use emblize::FrameParser;
use pyo3::prelude::*;

use crate::decode;

#[pyclass]
pub struct StreamDecoder{
    parser: FrameParser<'static, Vec<u8>>,
    out_buf_size: usize,
}

#[pymethods]
impl StreamDecoder {
    #[new]
    pub fn new(capacity: usize, sync: &[u8]) -> Self {
        let sync_vec: Box<[u8]> = sync.into();
        let sync_static: &'static [u8] = Box::leak(sync_vec);

        Self {
            parser: FrameParser::new(capacity, sync_static),
            out_buf_size: capacity / 2
        }
    }

    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<Py<PyAny>>, PyErr> {
        let mut decoded_frames = Vec::new();

        let (first, second) = self.parser.writable();

        let n = std::cmp::min(bytes.len(), first.len());
        first[..n].copy_from_slice(&bytes[..n]);

        if n < bytes.len() {
            let m = std::cmp::min(bytes.len() - n, second.len());
            second[..m].copy_from_slice(&bytes[n..n+m]);
        }

        let mut out = vec![0; self.out_buf_size];
        while let Some(length) = self.parser.poll_frame(&mut out) {
            decoded_frames.push(decode(&out[..length])?);
        }

        Ok(decoded_frames)
    }
}