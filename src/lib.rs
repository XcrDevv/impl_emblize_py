mod types;

use types::*;
use std::borrow::Cow;
use pyo3::{prelude::*, types::PyList};
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt};
use emblize::core::token::Token;

macro_rules! impl_py_to_token {
    ($($ty:ident),*) => {
        fn py_to_token<'py, 'a>(py: Python<'py>, obj: &Bound<'_, PyAny>, name: Option<Cow<'static, str>>) -> PyResult<Token<'a>> {
            match obj {
                obj if obj.is_instance_of::<PyDict>() => {
                    let mut tokens = vec![];
                    let dict = obj.downcast::<PyDict>()?;
                    for (key, value) in dict.iter() {
                        let k: Cow<'_, str> = Cow::Owned(key.extract::<String>()?);
                        tokens.push(py_to_token(py, &value, Some(k))?);
                    }
                    Ok(Token::Struct(name, tokens))
                }
                v if v.is_instance_of::<PyBool>() => {
                    let b: bool = v.extract()?;
                    Ok(Token::Bool(name, b))
                }
                v if v.is_instance_of::<PyList>() => {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "Unsupported list type. Instead use U8Arr([1, 2, 3]), U32Arr([1, 2, 3])"
                    ))
                }
                v if v.is_instance_of::<PyInt>() => {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "Python int is not safe for this binary format. Integers must have an explicit size. Use U8(5), U16(2), U32(6), etc."
                    ))
                }
                v if v.is_instance_of::<PyFloat>() => {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "Python float is not safe for this binary format. Integers must have an explicit size. Use U8(5), U16(2), U32(6), etc."
                    ))
                }
                $(
                    v if v.is_instance_of::<$ty>() => {
                        let py_ref: PyRef<$ty> = v.extract()?;
                        Ok(Token::$ty(name, py_ref.inner))
                    }
                )*
                v if v.is_instance_of::<Enum>() => {
                    let py_ref: PyRef<Enum> = v.extract()?;
                    let tk = py_to_token(py, &py_ref.inner.bind(py), name)?;
                    Ok(tk)
                }
                v if v.is_instance_of::<U8Arr>() => {
                    let py_ref: PyRef<U8Arr> = v.extract()?;
                    Ok(Token::U8Arr(name, py_ref.inner.clone().into()))
                }
                v if v.is_instance_of::<I32Arr>() => {
                    let py_ref: PyRef<I32Arr> = v.extract()?;
                    Ok(Token::I32Arr(name, py_ref.inner.clone().into()))
                }
                v if v.is_instance_of::<I64Arr>() => {
                    let py_ref: PyRef<I64Arr> = v.extract()?;
                    Ok(Token::I64Arr(name, py_ref.inner.clone().into()))
                }
                v if v.is_instance_of::<F32Arr>() => {
                    let py_ref: PyRef<F32Arr> = v.extract()?;
                    Ok(Token::F32Arr(name, py_ref.inner.clone().into()))
                }
                v if v.is_instance_of::<F64Arr>() => {
                    let py_ref: PyRef<F64Arr> = v.extract()?;
                    Ok(Token::F64Arr(name, py_ref.inner.clone().into()))
                }
                v if v.is_instance_of::<StrArr>() => {
                    let py_ref: PyRef<StrArr> = v.extract()?;
                    let vec: Vec<Cow<'static, str>> = py_ref.inner
                        .clone()
                        .into_iter()
                        .map(Cow::Owned)
                        .collect();
                    Ok(Token::StrArr(name, Cow::Owned(vec)))
                }
                _ => {
                    if let Some(name) = name {
                        return Err(pyo3::exceptions::PyTypeError::new_err(
                            format!("Unsupported type for key '{}'", name)
                        ))
                    } else {
                        return Err(pyo3::exceptions::PyTypeError::new_err("Unsupported type"))
                    }

                }
            }
        }
    }
}

macro_rules! impl_token_to_py {
    ($($variant:ident),*) => {
        fn token_to_py<'py, 'a>(token: &'a Token<'a>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            match token {
                Token::Struct(_name, fields) => {
                    let dict = PyDict::new(py);
                    for f in fields {
                        let k = f.name().to_string();
                        dict.set_item(k, token_to_py(f, py)?)?;
                    }
                    Ok(dict.into_pyobject(py)?.into_any())
                }

                Token::Enum(_, variant_index, value) => {
                    let dict = PyDict::new(py);
                    let idx = variant_index.into_pyobject(py)?;
                    let v = token_to_py(value.as_ref(), py)?;
                    dict.set_item("v_index".into_pyobject(py)?, idx)?;
                    dict.set_item("value".into_pyobject(py)?, v)?;
                    Ok(dict.into_pyobject(py)?.into_any())
                }
                Token::EmptyArr(_) => {
                    let empty: [u8; 0] = [];
                    Ok(empty.into_pyobject(py)?.to_owned().into_any())
                }
                Token::U8Arr(_, value) => {
                    Ok(value.clone().into_pyobject(py)?.to_owned().into_any())
                }
                Token::I32Arr(_, value) => {
                    Ok(value.clone().into_pyobject(py)?.to_owned().into_any())
                }
                Token::I64Arr(_, value) => {
                    Ok(value.clone().into_pyobject(py)?.to_owned().into_any())
                }
                Token::F32Arr(_, value) => {
                    Ok(value.clone().into_pyobject(py)?.to_owned().into_any())
                }
                Token::F64Arr(_, value) => {
                    Ok(value.clone().into_pyobject(py)?.to_owned().into_any())
                }
                Token::StrArr(_, value) => {
                    Ok(value.clone().into_pyobject(py)?.to_owned().into_any())
                }
                $(
                    Token::$variant(_, value) => Ok(value.into_pyobject(py)?.to_owned().into_any()),
                )*
            }
        }
    };
}

impl_py_to_token!(
    U8, U16, U32, U64, I8, I16, I32, I64, F32, F64, 
    TimestampMillis, TimestampMicros, MillisSinceBoot, MicrosSinceBoot, DurationMillis, DurationMicros, 
    Vec2, Vec3, Vec4, Quat
);

impl_token_to_py!(
    Bool, U8, U16, U32, U64, I8, I16, I32, I64, F32, F64, Str,
    TimestampMillis, TimestampMicros, MillisSinceBoot, MicrosSinceBoot, DurationMillis, DurationMicros,
    Vec2, Vec3, Vec4, Quat
);

#[pyfunction]
fn encode(obj: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    let token = Python::with_gil(|py| {
        py_to_token(py, &obj, None)
    });

    emblize::dynamic::encode(&token?)
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))
}

#[pyfunction]
fn decode<'py>(bytes: &[u8]) -> PyResult<PyObject> {
    let token: Token<'_> = Python::with_gil(|py| {
        py.allow_threads(|| {
            emblize::dynamic::decode(bytes)
        })
    })
    .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

    Python::with_gil(|py| {
        token_to_py(&token, py).map(|obj| obj.into())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn emblize_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;

    m.add_class::<U8>()?;
    m.add_class::<U16>()?;
    m.add_class::<U32>()?;
    m.add_class::<U64>()?;
    m.add_class::<I8>()?;
    m.add_class::<I16>()?;
    m.add_class::<I32>()?;
    m.add_class::<I64>()?;
    m.add_class::<F32>()?;
    m.add_class::<F64>()?;

    m.add_class::<TimestampMillis>()?;
    m.add_class::<TimestampMicros>()?;
    m.add_class::<MillisSinceBoot>()?;
    m.add_class::<MicrosSinceBoot>()?;

    m.add_class::<DurationMillis>()?;
    m.add_class::<DurationMicros>()?;

    m.add_class::<U8Arr>()?;
    m.add_class::<I32Arr>()?;
    m.add_class::<I64Arr>()?;
    m.add_class::<F32Arr>()?;
    m.add_class::<F64Arr>()?;
    m.add_class::<StrArr>()?;

    m.add_class::<Vec2>()?;
    m.add_class::<Vec3>()?;
    m.add_class::<Vec4>()?;
    m.add_class::<Quat>()?;
    Ok(())
}