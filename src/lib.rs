mod types;
mod frame;

use pyo3::IntoPyObjectExt;
use types::*;
use std::borrow::Cow;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyBytes, PyDict, PyFloat, PyInt, PyList, PyNone, PyString, PyType};
use emblize::core::token::{Token, TokenTag};

use crate::frame::StreamDecoder;

macro_rules! impl_tag_dtype_conversions {
    (
        pybuiltin: { $($builtin_variant:ident => $builtin_py:ty),* $(,)? },
        pyclass:   { $($class_variant:ident => $class_ty:ident),* $(,)? }
    ) => {
        fn token_tag_to_dtype<'py>(py: Python<'py>, tag: TokenTag) -> PyResult<Bound<'py, PyType>> {
            match tag {
                $(TokenTag::$builtin_variant => Ok(py.get_type::<$builtin_py>()),)*
                $(TokenTag::$class_variant   => Ok(py.get_type::<$class_ty>()),)*
                TokenTag::EmptyArr => Err(
                    PyErr::new::<pyo3::exceptions::PyTypeError, _>("Array is not a valid dtype")
                ),
            }
        }

        fn type_to_token_tag<'py>(py: Python<'py>, dtype: &Py<PyAny>) -> PyResult<TokenTag> {
            $(
                if dtype.is(&py.get_type::<$builtin_py>()) {
                    return Ok(TokenTag::$builtin_variant);
                }
            )*
            $(
                if dtype.is(&py.get_type::<$class_ty>()) {
                    return Ok(TokenTag::$class_variant);
                }
            )*
            Err(pyo3::exceptions::PyTypeError::new_err(
                format!("Unsupported dtype {:?}", dtype)
            ))
        }
    };
}

impl_tag_dtype_conversions!(
    pybuiltin: {
        Bool   => PyBool,
        Str    => PyString,
        Bytes  => PyBytes,
        None   => PyNone,
        Array  => PyList,
        Struct => PyDict,
    },
    pyclass: {
        U8              => U8,
        U16             => U16,
        U32             => U32,
        U64             => U64,
        I8              => I8,
        I16             => I16,
        I32             => I32,
        I64             => I64,
        F32             => F32,
        F64             => F64,
        Enum            => Enum,
        Some            => SomeValue,
        TimestampMillis => TimestampMillis,
        TimestampMicros => TimestampMicros,
        MillisSinceBoot => MillisSinceBoot,
        MicrosSinceBoot => MicrosSinceBoot,
        DurationMillis  => DurationMillis,
        DurationMicros  => DurationMicros,
        Vec2            => Vec2,
        Vec3            => Vec3,
        Vec4            => Vec4,
    }
);

macro_rules! match_type {
    ($dtype:expr, $py:expr, $t:ty) => {
        $dtype.is(&$py.get_type::<$t>()) || $dtype.is_instance_of::<$t>()
    };
}

fn f64_to_token<'py, 'a>(
    py: Python<'py>,
    value: f64,
    dtype: &Bound<'_, PyAny>
) -> PyResult<Token<'a>> {

    if match_type!(dtype, py, U8) {
        Ok(Token::U8(None, value as u8))
    } else if match_type!(dtype, py, U16) {
        Ok(Token::U16(None, value as u16))
    } else if match_type!(dtype, py, U32) {
        Ok(Token::U32(None, value as u32))
    } else if match_type!(dtype, py, U64) {
        Ok(Token::U64(None, value as u64))
    } else if match_type!(dtype, py, I8) {
        Ok(Token::I8(None, value as i8))
    } else if match_type!(dtype, py, I16) {
        Ok(Token::I16(None, value as i16))
    } else if match_type!(dtype, py, I32) {
        Ok(Token::I32(None, value as i32))
    } else if match_type!(dtype, py, I64) {
        Ok(Token::I64(None, value as i64))
    } else if match_type!(dtype, py, F32) {
        Ok(Token::F32(None, value as f32))
    } else if match_type!(dtype, py, F64) {
        Ok(Token::F64(None, value))
    } else if match_type!(dtype, py, TimestampMillis) {
        Ok(Token::TimestampMillis(None, value as u64))
    } else if match_type!(dtype, py, TimestampMicros) {
        Ok(Token::TimestampMicros(None, value as u64))
    } else if match_type!(dtype, py, MillisSinceBoot) {
        Ok(Token::MillisSinceBoot(None, value as u64))
    } else if match_type!(dtype, py, MicrosSinceBoot) {
        Ok(Token::MicrosSinceBoot(None, value as u64))
    } else if match_type!(dtype, py, DurationMillis) {
        Ok(Token::DurationMillis(None, value as i64))
    } else if match_type!(dtype, py, DurationMicros) {
        Ok(Token::DurationMicros(None, value as i64))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            format!("Unsupported dtype for number {:?}", dtype)
        ))
    }
} 

macro_rules! impl_py_to_token {
    ($($ty:ident),*) => {
        fn py_to_token<'py, 'a>(py: Python<'py>, obj: &Bound<'py, PyAny>, name: Option<Cow<'static, str>>, expected_type: Option<&Py<PyAny>>) -> PyResult<Token<'a>> {

            let ty = obj;

            match ty {
                ty if match_type!(ty, py, PyDict) => {
                    let mut tokens = vec![];
                    let dict = obj.downcast::<PyDict>()?;
                    for (key, value) in dict.iter() {
                        let k: Cow<'_, str> = Cow::Owned(key.extract::<String>()?);
                        tokens.push(py_to_token(py, &value, Some(k), None)?);
                    }
                    Ok(Token::Struct(name, tokens))
                }
                ty if match_type!(ty, py, PyNone) => {
                    Ok(Token::None(name))
                }
                ty if match_type!(ty, py, SomeValue) => {
                    let py_ref: PyRef<SomeValue> = obj.extract()?;
                    let tk = py_to_token(py, &py_ref.inner.bind(py), None, None)?;
                    Ok(Token::Some(name, Box::new(tk)))
                }
                ty if match_type!(ty, py, PyBool) => {
                    let b: bool = obj.extract()?;
                    Ok(Token::Bool(name, b))
                }
                ty if match_type!(ty, py, PyString) => {
                    let s: String = obj.extract()?;
                    Ok(Token::Str(name, s.into()))
                }
                ty if match_type!(ty, py, PyBytes) => {
                    let bound = obj.downcast::<PyBytes>()?;
                    Ok(Token::Bytes(name, bound.as_bytes().to_vec().into()))
                }
                ty if match_type!(ty, py, PyList) => {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "Unsupported list type. Instead use Array([...], dtype=...)"
                    ))
                }
                ty if match_type!(ty, py, Array) => {
                    let py_ref: PyRef<Array> = obj.extract()?;
                    
                    let expected_tag = type_to_token_tag(py, &py_ref.dtype)?;
                    
                    let mut collected = Vec::new();
                    for token in py_ref.inner.iter() {
                        let token = py_to_token(py, token.bind(py), None, Some(&py_ref.dtype))?;
                        let tag = TokenTag::from(&token);
                        if tag != expected_tag {
                             return Err(pyo3::exceptions::PyTypeError::new_err(
                                "All elements must be the same type"
                            ));
                        }
                        collected.push(token);
                    }

                    Ok(Token::Array(name, expected_tag, collected))
                }
                ty if match_type!(ty, py, PyInt) => {
                    let value: i64 = obj.extract()?;

                    match expected_type {
                        Some(t) => f64_to_token(py, value as f64, &t.into_bound_py_any(py)?),
                        None => Err(pyo3::exceptions::PyTypeError::new_err(
                            "Python int is not safe for this binary format. Integers must have an explicit size. Use U8(5), U16(2), U32(6), etc."
                        ))
                    }
                
                }
                ty if match_type!(ty, py, PyFloat) => {
                    let value: f64 = obj.extract()?;

                    match expected_type {
                        Some(t) => f64_to_token(py, value, &t.into_bound_py_any(py)?),
                        None => Err(pyo3::exceptions::PyTypeError::new_err(
                            "Python float is not safe for this binary format. Integers must have an explicit size. Use U8(5), U16(2), U32(6), etc."
                        ))
                    }
                }
                $(
                    ty if match_type!(ty, py, $ty) => {
                        let py_ref: PyRef<$ty> = obj.extract()?;
                        Ok(Token::$ty(name, py_ref.inner))
                    }
                )*
                ty if match_type!(ty, py, Enum) => {
                    let py_ref: PyRef<Enum> = obj.extract()?;
                    if let Some(inner) = &py_ref.inner {
                        let tk = py_to_token(py, &inner.bind(py), None, None)?;
                        Ok(Token::Enum(name, py_ref.variant_index, Some(Box::new(tk))))
                    } else {
                        Ok(Token::Enum(name, py_ref.variant_index, None))
                    }
                }
                ty if match_type!(ty, py, Vec2) => {
                    let py_ref: PyRef<Vec2> = obj.extract()?;

                    let tokens: Vec<Token> = py_ref.inner
                        .iter()
                        .map(|&val| f64_to_token(py, val, py_ref.dtype.bind(py)))
                        .collect::<PyResult<_>>()?;

                    Ok(Token::Vec2(name, Box::new(tokens.try_into().unwrap())))
                }
                ty if match_type!(ty, py, Vec3) => {
                    let py_ref: PyRef<Vec3> = obj.extract()?;

                    let tokens: Vec<Token> = py_ref.inner
                        .iter()
                        .map(|&val| f64_to_token(py, val, py_ref.dtype.bind(py)))
                        .collect::<PyResult<_>>()?;

                    Ok(Token::Vec3(name, Box::new(tokens.try_into().unwrap())))
                }
                ty if match_type!(ty, py, Vec4) => {
                    let py_ref: PyRef<Vec4> = obj.extract()?;

                    let tokens: Vec<Token> = py_ref.inner
                        .iter()
                        .map(|&val| f64_to_token(py, val, py_ref.dtype.bind(py)))
                        .collect::<PyResult<_>>()?;

                    Ok(Token::Vec4(name, Box::new(tokens.try_into().unwrap())))
                }
                _ => {
                    if let Some(name) = name {
                        return Err(pyo3::exceptions::PyTypeError::new_err(
                            format!("Unsupported type for key '{}'", name)
                        ))
                    } else {
                        return Err(pyo3::exceptions::PyTypeError::new_err("Unsupported type another test"))
                    }

                }
            }
        }
    }
}

fn token_to_f64(value: &Token) -> PyResult<f64> {
    match *value {
        Token::U8(_, v) => Ok(v as f64),
        Token::U16(_, v) => Ok(v as f64),
        Token::U32(_, v) => Ok(v as f64),
        Token::U64(_, v) => Ok(v as f64),
        Token::I8(_, v) => Ok(v as f64),
        Token::I16(_, v) => Ok(v as f64),
        Token::I32(_, v) => Ok(v as f64),
        Token::I64(_, v) => Ok(v as f64),
        Token::F32(_, v) => Ok(v as f64),
        Token::F64(_, v) => Ok(v as f64),
        _ => Err(pyo3::exceptions::PyTypeError::new_err("Unsupported type"))
    }
}

macro_rules! impl_token_to_py {
    (
        pybuiltin: {$($builtin_variant:ident),*},
        pyclass: {$($class_variant:ident),*},
        pyclassvec: {$($vec_variant:ident),*}
        
    ) => {
        fn token_to_py<'py, 'a>(py: Python<'py>, token: &'a Token<'a>) -> PyResult<Bound<'py, PyAny>> {
            match token {
                Token::Struct(_name, fields) => {
                    let dict = PyDict::new(py);
                    for f in fields {
                        let k = f.name().to_string();
                        dict.set_item(k, token_to_py(py, f)?)?;
                    }
                    Ok(dict.into_pyobject(py)?.into_any())
                }

                Token::Enum(_, index, value) => {
                    if let Some(value) = value {
                        let v = token_to_py(py, value.as_ref())?;
                        Ok(Enum {variant_index: *index, inner: Some(v.unbind().into_any()) }.into_bound_py_any(py)?)
                    } else {
                        Ok(Enum {variant_index: *index, inner: None }.into_bound_py_any(py)?)
                    }
                }
                Token::Some(_, value) => {
                    let v = token_to_py(py, value.as_ref())?;
                    Ok(v.into_bound_py_any(py)?)
                } 
                Token::None(_) => {
                    Ok(PyNone::get(py).into_bound_py_any(py)?)
                }
                Token::EmptyArr(_) => {
                    let empty: [u8; 0] = [];
                    Ok(empty.into_bound_py_any(py)?)
                }
                Token::Bytes(_, data) => {
                    Ok(PyBytes::new(py, data).into_any())
                }
                Token::Array(_, tag, values) => {
                    let collected: Vec<Py<PyAny>> = values.iter()
                        .map(|v| {
                            let inner = match v {
                                Token::U8(_, n) => n.into_bound_py_any(py)?,
                                Token::U16(_, n) => n.into_bound_py_any(py)?,
                                Token::U32(_, n) => n.into_bound_py_any(py)?,
                                Token::U64(_, n) => n.into_bound_py_any(py)?,
                                Token::I8(_, n) => n.into_bound_py_any(py)?,
                                Token::I16(_, n) => n.into_bound_py_any(py)?,
                                Token::I32(_, n) => n.into_bound_py_any(py)?,
                                Token::I64(_, n) => n.into_bound_py_any(py)?,
                                Token::F32(_, n) => n.into_bound_py_any(py)?,
                                Token::F64(_, n) => n.into_bound_py_any(py)?,
                                Token::Bool(_, b) => b.into_bound_py_any(py)?,
                                Token::Str(_, s) => s.into_bound_py_any(py)?,
                                other => token_to_py(py, other)?,
                            };
                            Ok(inner.unbind())
                        })
                        .collect::<PyResult<_>>()?;

                    let dtype = token_tag_to_dtype(py, *tag)?.unbind().into();
                    Ok(Array { inner: collected, dtype }.into_bound_py_any(py)?)
                }
                $(
                    Token::$builtin_variant(_, value) => Ok($builtin_variant { inner: *value}.into_bound_py_any(py)?),
                )*
                $(
                    Token::$class_variant(_, value) => Ok(value.into_pyobject(py)?.to_owned().into_any()),
                )*
                $(
                    Token::$vec_variant(_, values) => {
                        let vec: Vec<f64> = values.iter()
                            .map(|a| token_to_f64(a))
                            .collect::<PyResult<_>>()?;

                        let tag = TokenTag::from(&values[0]);
                        let dtype = token_tag_to_dtype(py, tag)?.unbind().into();
                        Ok($vec_variant { inner: vec.try_into().unwrap(), dtype }.into_bound_py_any(py)?)
                    }
                )*
            }
        }
    };
}

impl_py_to_token!(
    U8, U16, U32, U64, I8, I16, I32, I64, F32, F64, 
    TimestampMillis, TimestampMicros, MillisSinceBoot, MicrosSinceBoot, DurationMillis, DurationMicros
);

impl_token_to_py!(
    pybuiltin: {U8, U16, U32, U64, I8, I16, I32, I64, F32, F64},
    pyclass: {Bool, Str, TimestampMillis, TimestampMicros, MillisSinceBoot, MicrosSinceBoot, DurationMillis, DurationMicros},
    pyclassvec: {Vec2, Vec3, Vec4}
);

#[pyfunction]
fn encode(obj: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    let token = Python::with_gil(|py| {
        py_to_token(py, &obj, None, None)
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
        token_to_py(py, &token).map(|obj| obj.into())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn emblize_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;

    m.add_class::<StreamDecoder>()?;

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

    m.add_class::<Enum>()?;
    m.add_class::<SomeValue>()?;
    m.add_class::<Array>()?;

    m.add_class::<TimestampMillis>()?;
    m.add_class::<TimestampMicros>()?;
    m.add_class::<MillisSinceBoot>()?;
    m.add_class::<MicrosSinceBoot>()?;

    m.add_class::<DurationMillis>()?;
    m.add_class::<DurationMicros>()?;

    m.add_class::<Vec2>()?;
    m.add_class::<Vec3>()?;
    m.add_class::<Vec4>()?;
    Ok(())
}