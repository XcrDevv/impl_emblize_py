from typing import Any

def decode(bytes: bytes) -> Any:
    ...

def encode(obj: Any) -> bytes:
    ...
    
# Class definitions
# --------- Scalar ---------

class Infer: pass
    
class U8:
    inner: int
    def __init__(self, inner: int) -> None: ...

class U16:
    inner: int
    def __init__(self, inner: int) -> None: ...

class U32:
    inner: int
    def __init__(self, inner: int) -> None: ...

class U64:
    inner: int
    def __init__(self, inner: int) -> None: ...

class I8:
    inner: int
    def __init__(self, inner: int) -> None: ...

class I16:
    inner: int
    def __init__(self, inner: int) -> None: ...

class I32:
    inner: int
    def __init__(self, inner: int) -> None: ...

class I64:
    inner: int
    def __init__(self, inner: int) -> None: ...

class F32:
    inner: float
    def __init__(self, inner: float) -> None: ...

class F64:
    inner: float
    def __init__(self, inner: float) -> None: ...
    
# --------- Enum ---------
    
class Enum:
    variant_index: int
    inner: Any
    def __init__(self, variant_index: int, inner: Any) -> None: ...
    
# --------- Option ---------

class Some:
    inner: Any
    def __init__(self, inner: Any) -> None: ...
    
# --------- Time ---------
    
class TimestampMillis:
    inner: int
    def __init__(self, inner: int) -> None: ...

class TimestampMicros:
    inner: int
    def __init__(self, inner: int) -> None: ...

class MillisSinceBoot:
    inner: int
    def __init__(self, inner: int) -> None: ...

class MicrosSinceBoot:
    inner: int
    def __init__(self, inner: int) -> None: ...

class DurationMillis:
    inner: int
    def __init__(self, inner: int) -> None: ...

class DurationMicros:
    inner: int
    def __init__(self, inner: int) -> None: ...
    
# --------- Array ---------

class Array:
    inner: list[Any]
    def __init__(self, inner: list[Any], dtype) -> None: ...
    
# --------- Vector ---------
    
class Vec2:
    inner: tuple[float, float]
    def __init__(self, x: float, y: float, dtype) -> None: ...

class Vec3:
    inner: tuple[float, float, float]
    def __init__(self, x: float, y: float, z: float, dtype) -> None: ...

class Vec4:
    inner: tuple[float, float, float, float]
    def __init__(self, x: float, y: float, z: float, w: float, dtype) -> None: ...

class Quat:
    inner: tuple[float, float, float, float]
    def __init__(self, x: float, y: float, z: float, w: float) -> None: ...
