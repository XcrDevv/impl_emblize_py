use pyo3::prelude::*;

macro_rules! define_pyclass {
    ($name:ident, $type:ty) => {
        #[pyclass(frozen)]
        pub struct $name {
            pub inner: $type,
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new(inner: $type) -> Self {
                Self { inner }
            }
        }
    };
}

macro_rules! define_vec_pyclass {
    ($name:ident, $size:expr, $( $field:ident ),+ ) => {
        #[pyclass(frozen)]
        pub struct $name {
            pub inner: [f32; $size],
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new($( $field: f32 ),+) -> Self {
                Self {
                    inner: [$( $field ),+],
                }
            }
        }
    };
}

define_pyclass!(U8, u8);
define_pyclass!(U16, u16);
define_pyclass!(U32, u32);
define_pyclass!(U64, u64);
define_pyclass!(I8, i8);
define_pyclass!(I16, i16);
define_pyclass!(I32, i32);
define_pyclass!(I64, i64);
define_pyclass!(F32, f32);
define_pyclass!(F64, f64);

define_pyclass!(TimestampMillis, u64);
define_pyclass!(TimestampMicros, u64);
define_pyclass!(MillisSinceBoot, u64);
define_pyclass!(MicrosSinceBoot, u64);

define_pyclass!(DurationMillis, i64);
define_pyclass!(DurationMicros, i64);

define_pyclass!(U8Arr, Vec<u8>);
define_pyclass!(I32Arr, Vec<i32>);
define_pyclass!(I64Arr, Vec<i64>);
define_pyclass!(F32Arr, Vec<f32>);
define_pyclass!(F64Arr, Vec<f64>);
define_pyclass!(StrArr, Vec<String>);

define_vec_pyclass!(Vec2, 2, x, y);
define_vec_pyclass!(Vec3, 3, x, y, z);
define_vec_pyclass!(Vec4, 4, x, y, z, w);
define_vec_pyclass!(Quat, 4, x, y, z, w);

#[pyclass]
pub struct Enum {
    pub variant_index: u8,
    pub inner: Option<Py<PyAny>>,
}

#[pymethods]
impl Enum {
    #[new]
    #[pyo3(signature = (variant_index, value=None))]
    pub fn new(variant_index: u8, value: Option<Py<PyAny>>) -> Self {
        Self { variant_index, inner: value }
    } 
}