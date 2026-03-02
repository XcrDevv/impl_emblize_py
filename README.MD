# Usage

```py
import emblize_py as emblize

# Types like `str` and `bool` have a single canonical representation.
# Numeric values, however, can vary in size and signedness.
# Therefore we use explicit fixed-size types (e.g., U32) to guarantee
# a deterministic and well-defined binary representation.

encoded_data = emblize.encode({
    "id": emblize.U32(80)
    "device": "another test"
    "flag": True
})

data = emblize.decode(encoded_data)
```

## To-do

Right now Enums don't work properly, and even if they did, they would be difficult to use in Python.
Therefore, a conceptual model such as this is proposed:

```py
@emblize.enum
class E:
    @emblize.variant(0)
    class VariantA:
        def __init__(self, u: int):
            self.u = u
        
    @emblize.variant(1)
    class VariantB:
        def __init__(self, v: bool):
            self.v = v

decoded = emblize.decode(data, E)
```