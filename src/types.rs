use pyo3::prelude::*;

macro_rules! define_pyclass {
    ($name:ident, $type:ty) => {
        #[pyclass(frozen)]
        pub struct $name {
            #[pyo3(get)]
            pub inner: $type,
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new(inner: $type) -> Self {
                Self { inner }
            }

            fn __str__(&self) -> String {
                format!("{}", self.inner)
            }

            fn __repr__(&self) -> String {
                format!("{}({})", stringify!($name), self.inner)
            }
        }
    };
}

macro_rules! define_vec_pyclass {
    ($name:ident, $size:expr, $( $field:ident ),+ ) => {
        #[pyclass(frozen)]
        pub struct $name {
            #[pyo3(get)]
            pub inner: [f64; $size],

            #[pyo3(get)]
            pub dtype: Py<PyAny>
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new($( $field: f64 ),+, dtype: Py<PyAny>) -> Self {
                Self {
                    inner: [$( $field ),+],
                    dtype
                }
            }

            fn __str__(&self) -> String {
                let s = self.inner.map(|v| v.to_string()).join(", ");
                format!("({})", s)
            }

            fn __repr__(&self, py: Python<'_>) -> String {
                let components: Vec<String> = self.inner.iter().map(|v| v.to_string()).collect();

                let dtype_name = self.dtype.bind(py)
                    .getattr("__qualname__")
                    .and_then(|n| n.extract::<String>())
                    .unwrap_or_default();
                
                format!("{}({}, dtype={})", stringify!($name), components.join(", "), dtype_name)
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

define_vec_pyclass!(Vec2, 2, x, y);
define_vec_pyclass!(Vec3, 3, x, y, z);
define_vec_pyclass!(Vec4, 4, x, y, z, w);

#[pyclass(frozen)]
pub struct Enum {
    #[pyo3(get)]
    pub variant_index: u8,

    #[pyo3(get)]
    pub inner: Option<Py<PyAny>>,
}

#[pymethods]
impl Enum {
    #[new]
    #[pyo3(signature = (variant_index, value=None))]
    pub fn new(variant_index: u8, value: Option<Py<PyAny>>) -> Self {
        Self { variant_index, inner: value }
    }

    fn __str__(&self) -> String {
        if let Some(inner) = &self.inner {
            format!("Enum({}, {})", self.variant_index, inner)
        } else {
            format!("Enum({})", self.variant_index)
        }

    }

    fn __repr__(&self) -> String {
        if let Some(inner) = &self.inner {
            format!("Enum({}, {})", self.variant_index, inner)
        } else {
            format!("Enum({}, None)", self.variant_index)
        }
    }
}

#[pyclass(name = "Some", frozen)]
pub struct SomeValue {
    pub inner: Py<PyAny>
}

#[pymethods]
impl SomeValue {
    #[new]
    pub fn new(value: Py<PyAny>) -> Self {
        Self { inner: value }
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("Some({})", self.inner)
    }
}

#[pyclass(frozen)]
pub struct Array {
    #[pyo3(get)]
    pub inner: Vec<Py<PyAny>>,

    #[pyo3(get)]
    pub dtype: Py<PyAny>
}

#[pymethods]
impl Array {
    #[new]
    pub fn new(values: Vec<Py<PyAny>>, dtype: Py<PyAny>) -> Self {
        Self { inner: values, dtype }
    }

    fn __str__(&self) -> String {
        let s = self.inner
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        
        format!("[{}]", s)
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        let s = self.inner
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let dtype_name = self.dtype.bind(py)
            .getattr("__qualname__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_default();

        format!("Array[{}, dtype={}]", s, dtype_name)
    }
}