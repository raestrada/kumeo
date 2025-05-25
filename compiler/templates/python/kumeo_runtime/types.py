"""
Type definitions for the Kumeo Runtime Client
"""
from dataclasses import dataclass
from enum import Enum, auto
from typing import Any, Dict, Optional, Union, List, TypeVar, Generic, Type
from typing_extensions import TypedDict

# Type variable for generic message types
T = TypeVar('T')

class MessageType(Enum):
    """Types of messages that can be sent/received"""
    PING = auto()
    PONG = auto()
    RESOURCE_REQUEST = auto()
    RESOURCE_RESPONSE = auto()
    EVENT = auto()
    COMMAND = auto()
    
    @classmethod
    def from_str(cls, value: str) -> 'MessageType':
        """Convert string to MessageType"""
        try:
            return cls[value.upper()]
        except KeyError:
            raise ValueError(f"Invalid message type: {value}")

@dataclass
class Message(Generic[T]):
    """Base message class for all Kumeo protocol messages"""
    message_id: str
    message_type: MessageType
    payload: T
    timestamp: float
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert message to dictionary"""
        return {
            'message_id': self.message_id,
            'message_type': self.message_type.name,
            'payload': self.payload,
            'timestamp': self.timestamp
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'Message':
        """Create message from dictionary"""
        return cls(
            message_id=data['message_id'],
            message_type=MessageType.from_str(data['message_type']),
            payload=data['payload'],
            timestamp=data['timestamp']
        )

@dataclass
class ResourceRequest:
    """Resource request message"""
    resource_type: str
    resource_id: Optional[str] = None
    parameters: Optional[Dict[str, Any]] = None
    timeout: Optional[float] = None

@dataclass
class ResourceResponse:
    """Resource response message"""
    success: bool
    resource: Optional[Dict[str, Any]] = None
    error: Optional[str] = None

@dataclass
class Agent:
    """Agent information"""
    agent_id: str
    agent_type: str
    status: str
    metadata: Optional[Dict[str, Any]] = None

class ResourceSpec(TypedDict):
    """Resource specification"""
    api_version: str
    kind: str
    metadata: Dict[str, Any]
    spec: Dict[str, Any]
