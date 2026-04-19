`unstable!`

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

## Sequence

Arrays and Vec require an explicit `dtype` to determine how elements are encoded.
All elements must be of the same type — heterogeneous arrays are not supported.

```py
# Numeric arrays: elements are plain Python values, dtype carries the type info
encoded = emblize.encode(emblize.Array([10, 5, 2], dtype=emblize.U8))

# String arrays
encoded = emblize.encode(emblize.Array(["hello", "world"], dtype=str))

# Struct arrays
encoded = emblize.encode(emblize.Array([
    {"id": emblize.U8(1)},
    {"id": emblize.U8(2)},
], dtype=dict))
```

> [!NOTE]
> `dtype` is always required. Even for types with a single representation
> like `str`, the format does not infer the element type automatically.