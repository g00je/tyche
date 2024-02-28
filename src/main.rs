#[repr(C)]
#[derive(Debug, Default)]
struct CGene {
    id: u32,
    pepper: u16,
    server: u16,
}
impl CGene {
    const SIZE: usize = ::core::mem::size_of::<CGene>();
    fn is_none(&self) -> bool {
        let data: Vec<u8> = self.into();
        data.iter().all(|x| *x == 0)
    }
}
impl ::std::convert::From<&CGene> for Vec<u8> {
    fn from(value: &CGene) -> Self {
        unsafe {
            ::core::slice::from_raw_parts(
                value as *const CGene as *const u8,
                <CGene>::SIZE,
            )
            .iter()
            .map(|x| *x)
            .collect::<Vec<u8>>()
        }
    }
}
impl ::std::convert::From<CGene> for Vec<u8> {
    fn from(value: CGene) -> Self {
        unsafe {
            ::core::slice::from_raw_parts(
                &value as *const CGene as *const u8,
                <CGene>::SIZE,
            )
            .iter()
            .map(|x| *x)
            .collect::<Vec<u8>>()
        }
    }
}
impl ::std::convert::TryFrom<&[u8]> for CGene {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unsafe {
            let value: Result<[u8; <CGene>::SIZE], _> = value.try_into();
            match value {
                Err(_) => Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid input length",
                )),
                Ok(v) => Ok(::core::mem::transmute_copy(&v)),
            }
        }
    }
}
impl ::core::convert::TryFrom<Gene> for CGene {
    type Error = ::pyo3::PyErr;
    fn try_from(value: Gene) -> Result<Self, Self::Error> {
        Ok(Self { id: value.id, pepper: value.pepper, server: value.server })
    }
}
#[::pyo3::pyclass]
#[derive(Clone, Debug)]
pub struct Gene {
    #[pyo3(get)]
    id: u32,
    #[pyo3(get)]
    pepper: u16,
    #[pyo3(get)]
    server: u16,
}
impl ::core::convert::TryFrom<&CGene> for Gene {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &CGene) -> Result<Self, Self::Error> {
        Ok(Self { id: value.id, pepper: value.pepper, server: value.server })
    }
}
impl ::core::convert::TryFrom<&[u8]> for Gene {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value: Result<CGene, _> = value.try_into();
        match value {
            Err(_) => Err(::pyo3::exceptions::PyValueError::new_err(
                "invalid value to convert",
            )),
            Ok(value) => Ok((&value).try_into()?),
        }
    }
}
impl Gene {
    fn default() -> ::pyo3::PyResult<Self> {
        Ok(Self { id: 0, pepper: 0, server: 0 })
    }
    pub const PYI : & 'static
    str =
    "class Gene(plutus.Gene):\n    id: int\n    pepper: int\n    server: int\n\n    SIZE: int\n\n    def __new__(value: str | bytes = None) -> bytes: ...\n    def __bytes__(self) -> bytes: ...\n    def hex(self) -> str: ...\n    @classmethod\n    def batch(cls, value: str | bytes) -> list[Gene]: ...\n    def dict(self) -> str: ...\n"
    ;
}
#[::pyo3::pymethods]
impl Gene {
    #[classattr]
    pub const SIZE: u64 = <CGene>::SIZE as u64;
    #[new]
    fn py_new(value: Option<&::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
        match value {
            Some(value) => {
                if let Ok(m) = value.extract::<Gene>() {
                    return Ok(m);
                }
                let result: Result<Vec<u8>, _> =
                    value.extract::<Vec<u8>>().or_else(|_| {
                        match value.extract::<String>() {
                            Err(e) => Err(e),
                            Ok(v) => {
                                if v.len() != <CGene>::SIZE * 2 {
                                    return
                                Err(:: pyo3 :: exceptions :: PyValueError ::
                                new_err("invalid length")) ;
                                }
                                let v: Result<
                                    Vec<u8>,
                                    ::core::num::ParseIntError,
                                > = (0..v.len())
                                    .step_by(2)
                                    .map(|i| {
                                        u8::from_str_radix(&v[i..i + 2], 16)
                                    })
                                    .collect();
                                match v
                            {
                                Err(_) =>
                                Err(:: pyo3 :: exceptions :: PyValueError ::
                                new_err("invalid hex")), Ok(v) => Ok(v)
                            }
                            }
                        }
                    });
                if let Ok(data) = result {
                    let data = data.as_slice();
                    if data.len() != <CGene>::SIZE {
                        return Err(::pyo3::exceptions::PyValueError::new_err(
                            "invalid input length",
                        ));
                    }
                    let m: Gene = data.try_into()?;
                    return Ok(m);
                }
                Ok(Self::default()?)
            }
            None => Ok(Self::default()?),
        }
    }
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
    fn __bytes__(&self) -> ::pyo3::PyResult<::std::borrow::Cow<[u8]>> {
        let data: Vec<u8> = <CGene>::try_from(self.clone())?.into();
        Ok(data.to_owned().into())
    }
    fn __eq__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
        let a: Vec<u8> = <CGene>::try_from(self.clone())?.into();
        let b: Vec<u8> = <CGene>::try_from(other.clone())?.into();
        Ok(a == b)
    }
    fn hex(&self) -> ::pyo3::PyResult<String> {
        let data: Vec<u8> = <CGene>::try_from(self.clone())?.into();
        Ok(data.iter().map(|x| format!("{x:02x}")).collect())
    }
    #[classmethod]
    fn batch(
        _cls: &::pyo3::types::PyType, value: &::pyo3::PyAny,
    ) -> ::pyo3::PyResult<Vec<Self>> {
        let result: Result<Vec<u8>, _> =
            value.extract::<Vec<u8>>().or_else(|_| {
                match value.extract::<String>() {
                    Err(e) => Err(e),
                    Ok(v) => {
                        if v.len() != <CGene>::SIZE * 2 {
                            return Err(
                                ::pyo3::exceptions::PyValueError::new_err(
                                    "invalid hex length",
                                ),
                            );
                        }
                        let v: Result<Vec<u8>, ::core::num::ParseIntError> = (0
                            ..v.len())
                            .step_by(2)
                            .map(|i| u8::from_str_radix(&v[i..i + 2], 16))
                            .collect();
                        match v {
                            Err(_) => {
                                Err(::pyo3::exceptions::PyValueError::new_err(
                                    "invalid hex",
                                ))
                            }
                            Ok(v) => Ok(v),
                        }
                    }
                }
            });
        if let Ok(data) = result {
            let data = data.as_slice();
            if data.len() % <CGene>::SIZE != 0 {
                return Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid input length",
                ));
            }
            let total = data.len() / <CGene>::SIZE;
            let mut result: Vec<Gene> = Vec::with_capacity(total);
            for chunk in data.chunks(<CGene>::SIZE) {
                result.push(chunk.try_into()?);
            }
            return Ok(result);
        }
        Err(::pyo3::exceptions::PyValueError::new_err("invalid data"))
    }
    fn dict(&self) -> ::pyo3::PyResult<::std::string::String> {
        let data: Vec<u8> = <CGene>::try_from(self.clone())?.into();
        Ok(data
            .iter()
            .map(|x| format!("{x:02x}"))
            .collect::<::std::string::String>())
    }
    #[setter]
    fn id(&mut self, value: u32) -> ::pyo3::PyResult<()> {
        self.id = value;
        Ok(())
    }
    #[setter]
    fn pepper(&mut self, value: u16) -> ::pyo3::PyResult<()> {
        self.pepper = value;
        Ok(())
    }
    #[setter]
    fn server(&mut self, value: u16) -> ::pyo3::PyResult<()> {
        self.server = value;
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug, Default)]
struct CUser {
    gene: CGene,
    agent: [CGene; 3usize],
}
impl CUser {
    const SIZE: usize = ::core::mem::size_of::<CUser>();
}
impl ::std::convert::From<&CUser> for Vec<u8> {
    fn from(value: &CUser) -> Self {
        unsafe {
            ::core::slice::from_raw_parts(
                value as *const CUser as *const u8,
                <CUser>::SIZE,
            )
            .iter()
            .map(|x| *x)
            .collect::<Vec<u8>>()
        }
    }
}
impl ::std::convert::From<CUser> for Vec<u8> {
    fn from(value: CUser) -> Self {
        unsafe {
            ::core::slice::from_raw_parts(
                &value as *const CUser as *const u8,
                <CUser>::SIZE,
            )
            .iter()
            .map(|x| *x)
            .collect::<Vec<u8>>()
        }
    }
}
impl ::std::convert::TryFrom<&[u8]> for CUser {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unsafe {
            let value: Result<[u8; <CUser>::SIZE], _> = value.try_into();
            match value {
                Err(_) => Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid input length",
                )),
                Ok(v) => Ok(::core::mem::transmute_copy(&v)),
            }
        }
    }
}
impl ::core::convert::TryFrom<User> for CUser {
    type Error = ::pyo3::PyErr;
    fn try_from(value: User) -> Result<Self, Self::Error> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                gene: value.gene.try_borrow(py)?.clone().try_into()?,
                agent: {
                    let x = value.agent.map(|x| {
                        if let Some(v) = x {
                            v.try_borrow(py)?.clone().try_into()
                        } else {
                            Ok(<CGene>::default())
                        }
                    });
                    if let Some(_) = x.iter().find_map(|x| {
                        if x.is_err() {
                            Some(())
                        } else {
                            None
                        }
                    }) {
                        return Err(::pyo3::exceptions::PyValueError::new_err(
                            "could not convert the value",
                        ));
                    }
                    x.map(|x| x.unwrap())
                },
            })
        })
    }
}
#[::pyo3::pyclass]
#[derive(Clone, Debug)]
pub struct User {
    #[pyo3(get, set)]
    gene: ::pyo3::Py<Gene>,
    #[pyo3(get, set)]
    agent: [Option<::pyo3::Py<Gene>>; 3usize],
}
impl ::core::convert::TryFrom<&CUser> for User {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &CUser) -> Result<Self, Self::Error> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                gene: ::pyo3::Py::new(py, <Gene>::try_from(&value.gene)?)?,
                agent: [
                    {
                        let v = &value.agent[0usize];
                        if v.is_none() {
                            None
                        } else {
                            Some(::pyo3::Py::new(py, <Gene>::try_from(v)?)?)
                        }
                    },
                    {
                        let v = &value.agent[1usize];
                        if v.is_none() {
                            None
                        } else {
                            Some(::pyo3::Py::new(py, <Gene>::try_from(v)?)?)
                        }
                    },
                    {
                        let v = &value.agent[2usize];
                        if v.is_none() {
                            None
                        } else {
                            Some(::pyo3::Py::new(py, <Gene>::try_from(v)?)?)
                        }
                    },
                ],
            })
        })
    }
}
impl ::core::convert::TryFrom<&[u8]> for User {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value: Result<CUser, _> = value.try_into();
        match value {
            Err(_) => Err(::pyo3::exceptions::PyValueError::new_err(
                "invalid value to convert",
            )),
            Ok(value) => Ok((&value).try_into()?),
        }
    }
}
impl User {
    fn default() -> ::pyo3::PyResult<Self> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                gene: ::pyo3::Py::new(py, <Gene>::default()?)?,
                agent: [None, None, None],
            })
        })
    }
    pub const PYI: &'static str = "class User(plutus.User):\n    gene: Gene\n    agent: list[Gene]\n\n    SIZE: int\n\n    def __new__(value: str | bytes = None) -> bytes: ...\n    def __bytes__(self) -> bytes: ...\n    def hex(self) -> str: ...\n    @classmethod\n    def batch(cls, value: str | bytes) -> list[User]: ...\n    def dict(self) -> dict: ...\n";
}
#[::pyo3::pymethods]
impl User {
    #[classattr]
    pub const SIZE: u64 = <CUser>::SIZE as u64;
    #[new]
    fn py_new(value: Option<&::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
        match value {
            Some(value) => {
                if let Ok(m) = value.extract::<User>() {
                    return Ok(m);
                }
                let result: Result<Vec<u8>, _> =
                    value.extract::<Vec<u8>>().or_else(|_| {
                        match value.extract::<String>() {
                            Err(e) => Err(e),
                            Ok(v) => {
                                if v.len() != <CUser>::SIZE * 2 {
                                    return
                                Err(:: pyo3 :: exceptions :: PyValueError ::
                                new_err("invalid length")) ;
                                }
                                let v: Result<
                                    Vec<u8>,
                                    ::core::num::ParseIntError,
                                > = (0..v.len())
                                    .step_by(2)
                                    .map(|i| {
                                        u8::from_str_radix(&v[i..i + 2], 16)
                                    })
                                    .collect();
                                match v
                            {
                                Err(_) =>
                                Err(:: pyo3 :: exceptions :: PyValueError ::
                                new_err("invalid hex")), Ok(v) => Ok(v)
                            }
                            }
                        }
                    });
                if let Ok(data) = result {
                    let data = data.as_slice();
                    if data.len() != <CUser>::SIZE {
                        return Err(::pyo3::exceptions::PyValueError::new_err(
                            "invalid input length",
                        ));
                    }
                    let m: User = data.try_into()?;
                    return Ok(m);
                }
                Ok(Self::default()?)
            }
            None => Ok(Self::default()?),
        }
    }
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
    fn __bytes__(&self) -> ::pyo3::PyResult<::std::borrow::Cow<[u8]>> {
        let data: Vec<u8> = <CUser>::try_from(self.clone())?.into();
        Ok(data.to_owned().into())
    }
    fn __eq__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
        let a: Vec<u8> = <CUser>::try_from(self.clone())?.into();
        let b: Vec<u8> = <CUser>::try_from(other.clone())?.into();
        Ok(a == b)
    }
    fn hex(&self) -> ::pyo3::PyResult<String> {
        let data: Vec<u8> = <CUser>::try_from(self.clone())?.into();
        Ok(data.iter().map(|x| format!("{x:02x}")).collect())
    }
    #[classmethod]
    fn batch(
        _cls: &::pyo3::types::PyType, value: &::pyo3::PyAny,
    ) -> ::pyo3::PyResult<Vec<Self>> {
        let result: Result<Vec<u8>, _> =
            value.extract::<Vec<u8>>().or_else(|_| {
                match value.extract::<String>() {
                    Err(e) => Err(e),
                    Ok(v) => {
                        if v.len() != <CUser>::SIZE * 2 {
                            return Err(
                                ::pyo3::exceptions::PyValueError::new_err(
                                    "invalid hex length",
                                ),
                            );
                        }
                        let v: Result<Vec<u8>, ::core::num::ParseIntError> = (0
                            ..v.len())
                            .step_by(2)
                            .map(|i| u8::from_str_radix(&v[i..i + 2], 16))
                            .collect();
                        match v {
                            Err(_) => {
                                Err(::pyo3::exceptions::PyValueError::new_err(
                                    "invalid hex",
                                ))
                            }
                            Ok(v) => Ok(v),
                        }
                    }
                }
            });
        if let Ok(data) = result {
            let data = data.as_slice();
            if data.len() % <CUser>::SIZE != 0 {
                return Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid input length",
                ));
            }
            let total = data.len() / <CUser>::SIZE;
            let mut result: Vec<User> = Vec::with_capacity(total);
            for chunk in data.chunks(<CUser>::SIZE) {
                result.push(chunk.try_into()?);
            }
            return Ok(result);
        }
        Err(::pyo3::exceptions::PyValueError::new_err("invalid data"))
    }
    fn dict(&self) -> ::pyo3::PyResult<::pyo3::Py<::pyo3::types::PyDict>> {
        ::pyo3::Python::with_gil(|py| {
            let dict = ::pyo3::types::PyDict::new(py);
            dict.set_item("gene", self.gene.try_borrow(py)?.dict()?)?;
            dict.set_item(
                "agent",
                [
                    if let Some(v) = &self.agent[0usize] {
                        Some(v.try_borrow(py)?.dict()?)
                    } else {
                        None
                    },
                    if let Some(v) = &self.agent[1usize] {
                        Some(v.try_borrow(py)?.dict()?)
                    } else {
                        None
                    },
                    if let Some(v) = &self.agent[2usize] {
                        Some(v.try_borrow(py)?.dict()?)
                    } else {
                        None
                    },
                ],
            )?;
            Ok(dict.into())
        })
    }
}

fn main() {}
