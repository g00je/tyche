use std::num::ParseIntError;
use std::fmt::Write;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[repr(C)]
pub struct CGene {
    id: u32,
    pepper: u16,
    server: u16,
}

#[pyclass]
#[derive(Default, Clone, Copy, Debug)]
pub struct Gene {
    #[pyo3(get, set)]
    id: u32,
    #[pyo3(get, set)]
    pepper: u16,
    #[pyo3(get, set)]
    server: u16,
}

impl CGene {
    const SIZE: usize = ::std::mem::size_of::<CGene>();
}

/*

[u8] to struct:
&*(buf.as_ptr() as *const Struct)

struct to [u8]:
std::slice::from_raw_parts(
    &gene as *const _ as *const _,
    ::std::mem::size_of::<CGene>(),
)

::core::mem::transmute_copy(&::core::mem::ManuallyDrop::new($val))

*/

impl Gene {
    fn from_bytes(value: &[u8]) -> PyResult<Self> {
        if value.len() != CGene::SIZE {
            return Err(PyValueError::new_err("invalid value length."));
        }

        // let gene = value.align_to::<CGene>();
        let gene = unsafe { &*(value.as_ptr() as *const CGene) };

        Ok(Self { id: gene.id, pepper: gene.pepper, server: gene.server })
    }

    fn from_str(value: &str) -> PyResult<Self> {
        if value.len() != CGene::SIZE * 2 {
            return Err(PyValueError::new_err("invalid value length."));
        }

        let value: Result<Vec<u8>, ParseIntError> = (0..value.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
            .collect();

        let value = match value {
            Err(_) => return Err(PyValueError::new_err("invalid hex")),
            Ok(ar) => ar
        };

        // let gene = value.align_to::<CGene>();
        let gene = unsafe { &*(value.as_ptr() as *const CGene) };

        Ok(Self { id: gene.id, pepper: gene.pepper, server: gene.server })
    }
}

#[pymethods]
impl Gene {
    #[classattr]
    const SIZE: u64 = ::std::mem::size_of::<CGene>() as u64;

    #[new]
    fn py_new(value: Option<&PyAny>) -> PyResult<Self> {
        match value {
            Some(value) => {
                if let Ok(gene) = value.extract::<Gene>() {
                    return Ok(gene);
                }

                if let Ok(data) = value.extract::<&[u8]>() {
                    return Self::from_bytes(data);
                }

                if let Ok(data) = value.extract::<String>() {
                    return Self::from_str(&data);
                }

                Ok(Self::default())
            }
            None => Ok(Self::default()),
        }
    }

    fn __bytes__(&self) -> PyResult<&[u8]> {
        let gene =
            CGene { id: self.id, pepper: self.pepper, server: self.server };

        unsafe {
            Ok(::std::slice::from_raw_parts(
                &gene as *const _ as *const u8,
                ::std::mem::size_of::<CGene>(),
            ))
        }
    }

    fn __str__(&self) -> PyResult<String> {
        let gene =
        CGene { id: self.id, pepper: self.pepper, server: self.server };

        let bytes = unsafe {
            ::std::slice::from_raw_parts(
                &gene as *const _ as *const u8,
                ::std::mem::size_of::<CGene>(),
            )
        };

        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            write!(&mut s, "{:02x}", b).unwrap();
        }

        Ok(s)
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}
