"""
Kumeo Runtime Client implementation
"""
import asyncio
import json
import logging
import os
import socket
import time
import uuid
from pathlib import Path
from typing import Dict, Any, Optional, Callable, Awaitable, Union, List

from . import types
from .exceptions import (
    KumeoRuntimeError,
    ConnectionError,
    TimeoutError,
    ProtocolError,
    ResourceNotFoundError,
    PermissionDeniedError,
)

logger = logging.getLogger(__name__)

# Default socket path
DEFAULT_SOCKET_PATH = "/run/kumeo/runtime.sock"
# Default timeout in seconds
DEFAULT_TIMEOUT = 30
# Buffer size for socket reads
BUFFER_SIZE = 65536  # 64KB

MessageCallback = Callable[[types.Message], Awaitable[None]]

class RuntimeClient:
    """
    Client for interacting with the Kumeo runtime via Unix domain sockets.
    """
    
    def __init__(
        self,
        socket_path: str = DEFAULT_SOCKET_PATH,
        timeout: float = DEFAULT_TIMEOUT,
    ):
        """
        Initialize the runtime client.
        
        Args:
            socket_path: Path to the Unix domain socket
            timeout: Default timeout for operations in seconds
        """
        self.socket_path = socket_path
        self.timeout = timeout
        self.reader: Optional[asyncio.StreamReader] = None
        self.writer: Optional[asyncio.StreamWriter] = None
        self.message_handlers: Dict[types.MessageType, List[MessageCallback]] = {}
        self.pending_requests: Dict[str, asyncio.Future] = {}
        self.connected = False
        self.message_counter = 0
        
        # Register default message handlers
        self.register_message_handler(types.MessageType.PONG, self._handle_pong)
    
    async def connect(self) -> None:
        """
        Connect to the Kumeo runtime.
        
        Raises:
            ConnectionError: If the connection fails
        """
        if self.connected:
            return
            
        try:
            # Ensure the socket directory exists
            socket_dir = os.path.dirname(self.socket_path)
            Path(socket_dir).mkdir(parents=True, exist_ok=True)
            
            # Connect to the Unix domain socket
            self.reader, self.writer = await asyncio.wait_for(
                asyncio.open_unix_connection(self.socket_path),
                timeout=self.timeout
            )
            self.connected = True
            logger.info(f"Connected to Kumeo runtime at {self.socket_path}")
            
            # Start the message listener
            asyncio.create_task(self._message_listener())
            
            # Send a ping to verify the connection
            await self.ping()
            
        except asyncio.TimeoutError as e:
            raise TimeoutError(f"Connection to {self.socket_path} timed out") from e
        except (FileNotFoundError, ConnectionRefusedError) as e:
            raise ConnectionError(
                f"Failed to connect to Kumeo runtime at {self.socket_path}"
            ) from e
        except Exception as e:
            raise ConnectionError(
                f"Unexpected error connecting to Kumeo runtime: {str(e)}"
            ) from e
    
    async def close(self) -> None:
        """Close the connection to the Kumeo runtime."""
        if not self.connected:
            return
            
        try:
            if self.writer:
                self.writer.close()
                await self.writer.wait_closed()
        except Exception as e:
            logger.error(f"Error closing connection: {e}")
        finally:
            self.connected = False
            self.reader = None
            self.writer = None
    
    async def _message_listener(self) -> None:
        """Listen for incoming messages and dispatch them to handlers."""
        try:
            while self.connected and self.reader:
                try:
                    # Read the message length (first 4 bytes)
                    length_bytes = await self.reader.readexactly(4)
                    if not length_bytes:
                        break
                        
                    message_length = int.from_bytes(length_bytes, byteorder='big')
                    
                    # Read the message data
                    message_data = await self.reader.readexactly(message_length)
                    
                    # Parse the message
                    try:
                        message_dict = json.loads(message_data.decode('utf-8'))
                        message = types.Message.from_dict(message_dict)
                        
                        # Check if this is a response to a pending request
                        if message.message_id in self.pending_requests:
                            future = self.pending_requests.pop(message.message_id)
                            future.set_result(message)
                            continue
                        
                        # Dispatch to message handlers
                        await self._dispatch_message(message)
                        
                    except json.JSONDecodeError as e:
                        logger.error(f"Failed to decode message: {e}")
                    except Exception as e:
                        logger.error(f"Error processing message: {e}")
                        
                except asyncio.IncompleteReadError:
                    # Connection closed by the server
                    break
                except Exception as e:
                    logger.error(f"Error reading from socket: {e}")
                    break
                    
        except Exception as e:
            logger.error(f"Message listener error: {e}")
        finally:
            self.connected = False
            # Cancel all pending requests
            for future in self.pending_requests.values():
                if not future.done():
                    future.set_exception(ConnectionError("Connection closed"))
            self.pending_requests.clear()
    
    async def _dispatch_message(self, message: types.Message) -> None:
        """Dispatch a message to all registered handlers for its type."""
        handlers = self.message_handlers.get(message.message_type, [])
        for handler in handlers:
            try:
                await handler(message)
            except Exception as e:
                logger.error(f"Error in message handler: {e}", exc_info=True)
    
    def register_message_handler(
        self,
        message_type: types.MessageType,
        callback: MessageCallback,
    ) -> None:
        """
        Register a callback for a specific message type.
        
        Args:
            message_type: The type of message to handle
            callback: Async function to call when a message of the given type is received
        """
        if message_type not in self.message_handlers:
            self.message_handlers[message_type] = []
        self.message_handlers[message_type].append(callback)
    
    def unregister_message_handler(
        self,
        message_type: types.MessageType,
        callback: MessageCallback,
    ) -> None:
        """
        Unregister a message handler callback.
        
        Args:
            message_type: The message type to unregister from
            callback: The callback function to remove
        """
        if message_type in self.message_handlers:
            self.message_handlers[message_type] = [
                h for h in self.message_handlers[message_type] if h != callback
            ]
    
    async def _send_message(
        self,
        message_type: types.MessageType,
        payload: Any,
        message_id: Optional[str] = None,
        wait_for_response: bool = False,
        response_timeout: Optional[float] = None,
    ) -> Optional[types.Message]:
        """
        Send a message to the runtime.
        
        Args:
            message_type: Type of the message
            payload: Message payload (must be JSON-serializable)
            message_id: Optional message ID (auto-generated if not provided)
            wait_for_response: Whether to wait for a response
            response_timeout: Timeout in seconds to wait for a response
            
        Returns:
            The response message if wait_for_response is True, None otherwise
            
        Raises:
            ConnectionError: If not connected to the runtime
            TimeoutError: If the operation times out
            ProtocolError: If there is an error sending the message
        """
        if not self.connected or not self.writer:
            raise ConnectionError("Not connected to Kumeo runtime")
        
        message_id = message_id or str(uuid.uuid4())
        message = types.Message(
            message_id=message_id,
            message_type=message_type,
            payload=payload,
            timestamp=time.time(),
        )
        
        # Serialize the message
        try:
            message_data = json.dumps(message.to_dict()).encode('utf-8')
        except (TypeError, ValueError) as e:
            raise ProtocolError(f"Failed to serialize message: {e}") from e
        
        # Send the message length (4 bytes) followed by the message
        try:
            self.writer.write(len(message_data).to_bytes(4, byteorder='big'))
            self.writer.write(message_data)
            await self.writer.drain()
        except (ConnectionResetError, BrokenPipeError) as e:
            self.connected = False
            raise ConnectionError("Connection to Kumeo runtime lost") from e
        except Exception as e:
            raise ProtocolError(f"Failed to send message: {e}") from e
        
        if not wait_for_response:
            return None
            
        # Wait for the response
        future = asyncio.get_event_loop().create_future()
        self.pending_requests[message_id] = future
        
        try:
            response = await asyncio.wait_for(
                future,
                timeout=response_timeout or self.timeout
            )
            return response
        except asyncio.TimeoutError as e:
            self.pending_requests.pop(message_id, None)
            raise TimeoutError("Timed out waiting for response") from e
        except Exception as e:
            self.pending_requests.pop(message_id, None)
            raise
    
    # High-level API methods
    
    async def ping(self) -> float:
        """
        Send a ping to the runtime and measure the round-trip time.
        
        Returns:
            The round-trip time in seconds
            
        Raises:
            TimeoutError: If the ping times out
            ProtocolError: If there is a protocol error
        """
        start_time = time.monotonic()
        response = await self._send_message(
            types.MessageType.PING,
            {},
            wait_for_response=True
        )
        
        if not response or response.message_type != types.MessageType.PONG:
            raise ProtocolError("Invalid response to ping")
            
        return time.monotonic() - start_time
    
    async def _handle_pong(self, message: types.Message) -> None:
        """Handle PONG responses to PING messages."""
        if message.message_id in self.pending_requests:
            future = self.pending_requests.get(message.message_id)
            if future and not future.done():
                future.set_result(message)
    
    async def get_resource(
        self,
        resource_type: str,
        resource_id: Optional[str] = None,
        parameters: Optional[Dict[str, Any]] = None,
        timeout: Optional[float] = None,
    ) -> Dict[str, Any]:
        """
        Get a resource from the runtime.
        
        Args:
            resource_type: Type of the resource to get
            resource_id: Optional ID of the specific resource
            parameters: Optional parameters for the request
            timeout: Optional timeout in seconds
            
        Returns:
            The requested resource
            
        Raises:
            ResourceNotFoundError: If the resource is not found
            PermissionDeniedError: If the operation is not permitted
            TimeoutError: If the operation times out
            ProtocolError: If there is a protocol error
        """
        request = types.ResourceRequest(
            resource_type=resource_type,
            resource_id=resource_id,
            parameters=parameters or {},
            timeout=timeout
        )
        
        response = await self._send_message(
            types.MessageType.RESOURCE_REQUEST,
            request.__dict__,
            wait_for_response=True,
            response_timeout=timeout
        )
        
        if not response or 'success' not in response.payload:
            raise ProtocolError("Invalid response from runtime")
        
        if not response.payload['success']:
            error = response.payload.get('error', 'Unknown error')
            if 'not found' in error.lower():
                raise ResourceNotFoundError(error)
            if 'permission denied' in error.lower():
                raise PermissionDeniedError(error)
            raise KumeoRuntimeError(error)
        
        return response.payload.get('resource', {})
    
    async def list_agents(self) -> List[types.Agent]:
        """
        Get a list of all registered agents.
        
        Returns:
            List of agent information
        """
        try:
            resources = await self.get_resource("agents")
            return [
                types.Agent(
                    agent_id=agent_data['agent_id'],
                    agent_type=agent_data['agent_type'],
                    status=agent_data.get('status', 'unknown'),
                    metadata=agent_data.get('metadata')
                )
                for agent_data in resources.get('items', [])
            ]
        except Exception as e:
            logger.error(f"Failed to list agents: {e}")
            raise
    
    # Context manager support
    async def __aenter__(self) -> 'RuntimeClient':
        await self.connect()
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:
        await self.close()
