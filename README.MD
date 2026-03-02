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