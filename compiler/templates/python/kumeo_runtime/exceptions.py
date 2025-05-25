"""
Exceptions for the Kumeo Runtime Client
"""

class KumeoRuntimeError(Exception):
    """Base exception for all Kumeo runtime errors"""
    pass

class ConnectionError(KumeoRuntimeError):
    """Raised when there is an error connecting to the runtime"""
    pass

class TimeoutError(KumeoRuntimeError):
    """Raised when an operation times out"""
    pass

class ProtocolError(KumeoRuntimeError):
    """Raised when there is an error in the protocol communication"""
    pass

class ResourceNotFoundError(KumeoRuntimeError):
    """Raised when a requested resource is not found"""
    pass

class PermissionDeniedError(KumeoRuntimeError):
    """Raised when the operation is not permitted"""
    pass
