from typing import Any, TypeVar

def encode(obj: Any) -> bytes:
    """
    Encode a Python object into the emblize binary format.

    Dicts are encoded as structs, typed wrappers (e.g. U8, Array)
    preserve their explicit type information. Plain Python ints and
    floats are not accepted — use explicit types like U8(5) or F32(1.0).
    """
    ...
    
def decode(data: bytes) -> Any:
    """
    Decode emblize binary data into a Python object.
    """
    ...
    
# Class definitions
# --------- Scalar ---------

class U8():
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class U16:
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class U32:
    """ 
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class U64:
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class I8:
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class I16:
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class I32:
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class I64:
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class F32:
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: float
    def __init__(self, inner: float) -> None: ...

class F64:
    """
    A fixed-size numeric type for use with the emblize binary format.

    Use these wrappers instead of plain Python ints or floats to guarantee
    a deterministic binary representation.
    """
    inner: float
    def __init__(self, inner: float) -> None: ...
    
# --------- Enum ---------
    
class Enum:
    """
    Represents an enum variant, optionally carrying an inner value.

    :param variant_index: The index of the variant (0–127).
    :param inner: The associated value, or None for unit variants.
    """
    variant_index: int
    inner: Any
    def __init__(self, variant_index: int, inner: Any) -> None: ...
    
# --------- Option ---------

class Some:
    """
    Represents an optional value that is present.

    Encodes as the emblize Some token. Use Python's `None` directly
    for the absent case.
    """
    inner: Any
    def __init__(self, inner: Any) -> None: ...
    
# --------- Time ---------


TIME_DOC = """
A time-related value for use with the emblize binary format.

:param inner: The raw integer value representing the timestamp or duration.
"""
    
class TimestampMillis:
    """
    A time-related value for use with the emblize binary format.

    :param inner: The raw unsigned integer value representing the timestamp.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class TimestampMicros:
    """
    A time-related value for use with the emblize binary format.

    :param inner: The raw unsigned integer value representing the timestamp.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class MillisSinceBoot:
    """
    A time-related value for use with the emblize binary format.

    :param inner: The raw unsigned integer value representing the time.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class MicrosSinceBoot:
    """
    A time-related value for use with the emblize binary format.

    :param inner: The raw unsigned integer value representing the time.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class DurationMillis:
    """
    A time-related value for use with the emblize binary format.

    :param inner: The raw signed integer value representing the time.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...

class DurationMicros:
    """
    A time-related value for use with the emblize binary format.

    :param inner: The raw signed integer value representing the time.
    """
    inner: int
    def __init__(self, inner: int) -> None: ...
    
# --------- Array ---------

class Array:
    """
    A typed homogeneous array.

    All elements must match the given dtype. Plain Python ints and floats
    are accepted as elements when dtype is a numeric type (e.g. U8, F32, str, dict).
    """
    inner: list[Any]
    def __init__(self, inner: list[Any], dtype) -> None: ...
    
# --------- Vector ---------
    
class Vec2:
    """
    A 2-component numeric vector.

    :param dtype: The numeric type of the components (e.g. F32).
    """
    inner: tuple[float, float]
    def __init__(self, x: float, y: float, dtype) -> None: ...

class Vec3:
    """
    A 3-component numeric vector.

    :param dtype: The numeric type of the components (e.g. F32).
    """
    inner: tuple[float, float, float]
    def __init__(self, x: float, y: float, z: float, dtype) -> None: ...

class Vec4:
    """
    A 4-component numeric vector.

    :param dtype: The numeric type of the components (e.g. F32).
    """
    inner: tuple[float, float, float, float]
    def __init__(self, x: float, y: float, z: float, w: float, dtype) -> None: ...

# Stream Decoder
# --------- Decoder ---------

T = TypeVar("T")

class StreamDecoder:
    """# Example
    ```python
    decoder = emblize.StreamDecoder(size=1024, sync=""""b'\\xAA\\xBB'"""")

    for frame in decoder.push(data):
        print(frame)
    ```
    A streaming decoder for framed emblize data.
    """
    def __init__(self, size: int, sync: bytes) -> None:
        """
        Create a new StreamDecoder.
        
        :param size: Internal ring buffer capacity in bytes.
        :param sync: Byte sequence used to detect frame boundaries.
        """
        ...
    
    def push[T](self, data: bytes) -> list[T]:
        """
        Push a chunk of bytes into the decoder.
        Returns a list of decoded objects, empty if no complete frame was found yet.
        """
        ...