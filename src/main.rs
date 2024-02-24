#[repr(C)]
#[derive(Debug)]
struct CGene {
    id: u32,
    pepper: u16,
    server: u16,
}
impl CGene {
    const SIZE: usize = ::core::mem::size_of::<CGene>();
}
impl ::std::convert::From<CGene> for &[u8] {
    fn from(value: CGene) -> Self {
        unsafe {
            ::core::slice::from_raw_parts(
                &value as *const CGene as *const u8,
                <CGene>::SIZE,
            )
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
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                id: value.id,
                pepper: value.pepper,
                server: value.server,
            })
        })
    }
}
#[::pyo3::pyclass]
#[derive(Clone, Debug)]
struct Gene {
    #[pyo3(get, set)]
    id: u32,
    #[pyo3(get, set)]
    pepper: u16,
    #[pyo3(get, set)]
    server: u16,
}
impl ::core::convert::TryFrom<CGene> for Gene {
    type Error = ::pyo3::PyErr;
    fn try_from(value: CGene) -> Result<Self, Self::Error> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                id: value.id,
                pepper: value.pepper,
                server: value.server,
            })
        })
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
            Ok(value) => Ok(value.try_into()?),
        }
    }
}
impl ::core::convert::TryFrom<&str> for Gene {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != <CGene>::SIZE * 2 {
            return Err(::pyo3::exceptions::PyValueError::new_err(
                "invalid length",
            ));
        }
        let value: Result<Vec<u8>, ::core::num::ParseIntError> = (0..value
            .len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
            .collect();
        let value = match value {
            Err(_) => {
                return Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid hex",
                ))
            }
            Ok(v) => v,
        };
        let value: CGene = value.as_slice().try_into()?;
        Ok(value.try_into()?)
    }
}
impl Gene {
    fn default() -> ::pyo3::PyResult<Self> {
        ::pyo3::Python::with_gil(|py| Ok(Self { id: 0, pepper: 0, server: 0 }))
    }
}
#[::pyo3::pymethods]
impl Gene {
    #[classattr]
    const SIZE: u64 = <CGene>::SIZE as u64;
    #[new]
    fn py_new(value: Option<&::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
        match value {
            Some(value) => {
                if let Ok(m) = value.extract::<Gene>() {
                    return Ok(m);
                }
                if let Ok(data) = value.extract::<&[u8]>() {
                    let m: Result<Gene, _> = data.try_into();
                    return match m {
                        Ok(m) => Ok(m),
                        Err(e) => {
                            Err(::pyo3::exceptions::PyValueError::new_err(e))
                        }
                    };
                }
                if let Ok(data) = value.extract::<String>() {
                    let m: Result<Gene, _> = data.as_str().try_into();
                    return match m {
                        Ok(m) => Ok(m),
                        Err(e) => {
                            Err(::pyo3::exceptions::PyValueError::new_err(e))
                        }
                    };
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
        let data: &[u8] = <CGene>::try_from(self.clone())?.into();
        Ok(data.to_owned().into())
    }
    fn __eq__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
        let a: &[u8] = <CGene>::try_from(self.clone())?.into();
        let b: &[u8] = <CGene>::try_from(other.clone())?.into();
        Ok(a == b)
    }
}
#[repr(C)]
#[derive(Debug)]
struct CSessionInfo {
    client: u8,
    os: u8,
    browser: u8,
    device: u8,
    client_version: u16,
    os_version: u16,
    browser_version: u16,
    _reserved: u16,
}
impl CSessionInfo {
    const SIZE: usize = ::core::mem::size_of::<CSessionInfo>();
}
impl ::std::convert::From<CSessionInfo> for &[u8] {
    fn from(value: CSessionInfo) -> Self {
        unsafe {
            ::core::slice::from_raw_parts(
                &value as *const CSessionInfo as *const u8,
                <CSessionInfo>::SIZE,
            )
        }
    }
}
impl ::std::convert::TryFrom<&[u8]> for CSessionInfo {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unsafe {
            let value: Result<[u8; <CSessionInfo>::SIZE], _> = value.try_into();
            match value {
                Err(_) => Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid input length",
                )),
                Ok(v) => Ok(::core::mem::transmute_copy(&v)),
            }
        }
    }
}
impl ::core::convert::TryFrom<SessionInfo> for CSessionInfo {
    type Error = ::pyo3::PyErr;
    fn try_from(value: SessionInfo) -> Result<Self, Self::Error> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                client: value.client,
                os: value.os,
                browser: value.browser,
                device: value.device,
                client_version: value.client_version,
                os_version: value.os_version,
                browser_version: value.browser_version,
                _reserved: value._reserved,
            })
        })
    }
}
#[::pyo3::pyclass]
#[derive(Clone, Debug)]
struct SessionInfo {
    #[pyo3(get, set)]
    client: u8,
    #[pyo3(get, set)]
    os: u8,
    #[pyo3(get, set)]
    browser: u8,
    #[pyo3(get, set)]
    device: u8,
    #[pyo3(get, set)]
    client_version: u16,
    #[pyo3(get, set)]
    os_version: u16,
    #[pyo3(get, set)]
    browser_version: u16,
    #[pyo3(get, set)]
    _reserved: u16,
}
impl ::core::convert::TryFrom<CSessionInfo> for SessionInfo {
    type Error = ::pyo3::PyErr;
    fn try_from(value: CSessionInfo) -> Result<Self, Self::Error> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                client: value.client,
                os: value.os,
                browser: value.browser,
                device: value.device,
                client_version: value.client_version,
                os_version: value.os_version,
                browser_version: value.browser_version,
                _reserved: value._reserved,
            })
        })
    }
}
impl ::core::convert::TryFrom<&[u8]> for SessionInfo {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value: Result<CSessionInfo, _> = value.try_into();
        match value {
            Err(_) => Err(::pyo3::exceptions::PyValueError::new_err(
                "invalid value to convert",
            )),
            Ok(value) => Ok(value.try_into()?),
        }
    }
}
impl ::core::convert::TryFrom<&str> for SessionInfo {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != <CSessionInfo>::SIZE * 2 {
            return Err(::pyo3::exceptions::PyValueError::new_err(
                "invalid length",
            ));
        }
        let value: Result<Vec<u8>, ::core::num::ParseIntError> = (0..value
            .len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
            .collect();
        let value = match value {
            Err(_) => {
                return Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid hex",
                ))
            }
            Ok(v) => v,
        };
        let value: CSessionInfo = value.as_slice().try_into()?;
        Ok(value.try_into()?)
    }
}
impl SessionInfo {
    fn default() -> ::pyo3::PyResult<Self> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                client: 0,
                os: 0,
                browser: 0,
                device: 0,
                client_version: 0,
                os_version: 0,
                browser_version: 0,
                _reserved: 0,
            })
        })
    }
}
#[::pyo3::pymethods]
impl SessionInfo {
    #[classattr]
    const SIZE: u64 = <CSessionInfo>::SIZE as u64;
    #[new]
    fn py_new(value: Option<&::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
        match value {
            Some(value) => {
                if let Ok(m) = value.extract::<SessionInfo>() {
                    return Ok(m);
                }
                if let Ok(data) = value.extract::<&[u8]>() {
                    let m: Result<SessionInfo, _> = data.try_into();
                    return match m {
                        Ok(m) => Ok(m),
                        Err(e) => {
                            Err(::pyo3::exceptions::PyValueError::new_err(e))
                        }
                    };
                }
                if let Ok(data) = value.extract::<String>() {
                    let m: Result<SessionInfo, _> = data.as_str().try_into();
                    return match m {
                        Ok(m) => Ok(m),
                        Err(e) => {
                            Err(::pyo3::exceptions::PyValueError::new_err(e))
                        }
                    };
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
        let data: &[u8] = <CSessionInfo>::try_from(self.clone())?.into();
        Ok(data.to_owned().into())
    }
    fn __eq__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
        let a: &[u8] = <CSessionInfo>::try_from(self.clone())?.into();
        let b: &[u8] = <CSessionInfo>::try_from(other.clone())?.into();
        Ok(a == b)
    }
}
#[repr(C)]
#[derive(Debug)]
struct CSession {
    ip: [u8; 4usize],
    info: CSessionInfo,
    timestamp: u64,
    token: [u8; 64usize],
}
impl CSession {
    const SIZE: usize = ::core::mem::size_of::<CSession>();
}
impl ::std::convert::From<CSession> for &[u8] {
    fn from(value: CSession) -> Self {
        unsafe {
            ::core::slice::from_raw_parts(
                &value as *const CSession as *const u8,
                <CSession>::SIZE,
            )
        }
    }
}
impl ::std::convert::TryFrom<&[u8]> for CSession {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unsafe {
            let value: Result<[u8; <CSession>::SIZE], _> = value.try_into();
            match value {
                Err(_) => Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid input length",
                )),
                Ok(v) => Ok(::core::mem::transmute_copy(&v)),
            }
        }
    }
}
impl ::core::convert::TryFrom<Session> for CSession {
    type Error = ::pyo3::PyErr;
    fn try_from(value: Session) -> Result<Self, Self::Error> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                ip: value.ip,
                info: value.info.try_borrow(py)?.clone().try_into()?,
                timestamp: value.timestamp,
                token: value.token,
            })
        })
    }
}
#[::pyo3::pyclass]
#[derive(Clone, Debug)]
struct Session {
    ip: [u8; 4usize],
    #[pyo3(get, set)]
    info: ::pyo3::Py<SessionInfo>,
    #[pyo3(get, set)]
    timestamp: u64,
    token: [u8; 64usize],
}
impl ::core::convert::TryFrom<CSession> for Session {
    type Error = ::pyo3::PyErr;
    fn try_from(value: CSession) -> Result<Self, Self::Error> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                ip: value.ip,
                info: {
                    let v: SessionInfo = value.info.try_into()?;
                    ::pyo3::Py::new(py, v)?
                },
                timestamp: value.timestamp,
                token: value.token,
            })
        })
    }
}
impl ::core::convert::TryFrom<&[u8]> for Session {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value: Result<CSession, _> = value.try_into();
        match value {
            Err(_) => Err(::pyo3::exceptions::PyValueError::new_err(
                "invalid value to convert",
            )),
            Ok(value) => Ok(value.try_into()?),
        }
    }
}
impl ::core::convert::TryFrom<&str> for Session {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != <CSession>::SIZE * 2 {
            return Err(::pyo3::exceptions::PyValueError::new_err(
                "invalid length",
            ));
        }
        let value: Result<Vec<u8>, ::core::num::ParseIntError> = (0..value
            .len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
            .collect();
        let value = match value {
            Err(_) => {
                return Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid hex",
                ))
            }
            Ok(v) => v,
        };
        let value: CSession = value.as_slice().try_into()?;
        Ok(value.try_into()?)
    }
}
impl Session {
    fn default() -> ::pyo3::PyResult<Self> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                ip: [0; 4usize],
                info: ::pyo3::Py::new(py, <SessionInfo>::default()?)?,
                timestamp: 0,
                token: [0; 64usize],
            })
        })
    }
}
#[::pyo3::pymethods]
impl Session {
    #[classattr]
    const SIZE: u64 = <CSession>::SIZE as u64;
    #[new]
    fn py_new(value: Option<&::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
        match value {
            Some(value) => {
                if let Ok(m) = value.extract::<Session>() {
                    return Ok(m);
                }
                if let Ok(data) = value.extract::<&[u8]>() {
                    let m: Result<Session, _> = data.try_into();
                    return match m {
                        Ok(m) => Ok(m),
                        Err(e) => {
                            Err(::pyo3::exceptions::PyValueError::new_err(e))
                        }
                    };
                }
                if let Ok(data) = value.extract::<String>() {
                    let m: Result<Session, _> = data.as_str().try_into();
                    return match m {
                        Ok(m) => Ok(m),
                        Err(e) => {
                            Err(::pyo3::exceptions::PyValueError::new_err(e))
                        }
                    };
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
        let data: &[u8] = <CSession>::try_from(self.clone())?.into();
        Ok(data.to_owned().into())
    }
    fn __eq__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
        let a: &[u8] = <CSession>::try_from(self.clone())?.into();
        let b: &[u8] = <CSession>::try_from(other.clone())?.into();
        Ok(a == b)
    }
    #[getter]
    fn get_ip(&self) -> &[u8] {
        &self.ip
    }
    #[setter]
    fn set_ip(&mut self, value: &[u8]) -> ::pyo3::PyResult<()> {
        if value.len() != 4usize {
            return Err(::pyo3::exceptions::PyValueError::new_err(format!(
                "input length must be {}",
                4usize
            )));
        }
        self.ip = match value.try_into() {
            Err(_) => {
                return Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid input",
                ))
            }
            Ok(v) => v,
        };
        Ok(())
    }
    #[getter]
    fn get_token(&self) -> &[u8] {
        &self.token
    }
    #[setter]
    fn set_token(&mut self, value: &[u8]) -> ::pyo3::PyResult<()> {
        if value.len() != 64usize {
            return Err(::pyo3::exceptions::PyValueError::new_err(format!(
                "input length must be {}",
                64usize
            )));
        }
        self.token = match value.try_into() {
            Err(_) => {
                return Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid input",
                ))
            }
            Ok(v) => v,
        };
        Ok(())
    }
}
#[repr(C)]
#[derive(Debug)]
struct CUser {
    flag: u64,
    gene: CGene,
    agent: CGene,
    review: CGene,
    photo: CGene,
    phone: [u8; 12usize],
    cc: u16,
    name: [u8; 50usize],
    sessions: [CSession; 3usize],
    mat: [[CGene; 3usize]; 3usize],
}
impl CUser {
    const SIZE: usize = ::core::mem::size_of::<CUser>();
}
impl ::std::convert::From<CUser> for &[u8] {
    fn from(value: CUser) -> Self {
        unsafe {
            ::core::slice::from_raw_parts(
                &value as *const CUser as *const u8,
                <CUser>::SIZE,
            )
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
                flag: value.flag,
                gene: value.gene.try_borrow(py)?.clone().try_into()?,
                agent: value.agent.try_borrow(py)?.clone().try_into()?,
                review: value.review.try_borrow(py)?.clone().try_into()?,
                photo: value.photo.try_borrow(py)?.clone().try_into()?,
                phone: {
                    let mut data = value.phone.as_bytes().to_vec();
                    data.resize(12usize, 0);
                    data.as_slice().try_into().unwrap()
                },
                cc: value.cc,
                name: {
                    let mut data = value.name.as_bytes().to_vec();
                    data.resize(50usize, 0);
                    data.as_slice().try_into().unwrap()
                },
                sessions: {
                    let k = value.sessions.map(|x| {
                        Some(CSession::try_from(x))
                    });

                },
                mat: value.mat.map(|x| x.map(|x| x.try_into()?)),
            })
        })
    }
}
#[::pyo3::pyclass]
#[derive(Clone, Debug)]
struct User {
    #[pyo3(get, set)]
    flag: u64,
    #[pyo3(get, set)]
    gene: ::pyo3::Py<Gene>,
    #[pyo3(get, set)]
    agent: ::pyo3::Py<Gene>,
    #[pyo3(get, set)]
    review: ::pyo3::Py<Gene>,
    #[pyo3(get, set)]
    photo: ::pyo3::Py<Gene>,
    #[pyo3(get)]
    phone: String,
    #[pyo3(get, set)]
    cc: u16,
    #[pyo3(get)]
    name: String,
    #[pyo3(get, set)]
    sessions: [Session; 3usize],
    #[pyo3(get, set)]
    mat: [[Gene; 3usize]; 3usize],
}
impl ::core::convert::TryFrom<CUser> for User {
    type Error = ::pyo3::PyErr;
    fn try_from(value: CUser) -> Result<Self, Self::Error> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                flag: value.flag,
                gene: {
                    let v: Gene = value.gene.try_into()?;
                    ::pyo3::Py::new(py, v)?
                },
                agent: {
                    let v: Gene = value.agent.try_into()?;
                    ::pyo3::Py::new(py, v)?
                },
                review: {
                    let v: Gene = value.review.try_into()?;
                    ::pyo3::Py::new(py, v)?
                },
                photo: {
                    let v: Gene = value.photo.try_into()?;
                    ::pyo3::Py::new(py, v)?
                },
                phone: ::std::string::String::from_utf8(value.phone.to_vec())
                    .unwrap_or_else(|e| {
                        ::std::string::String::from_utf8(
                            value.phone[..e.utf8_error().valid_up_to()].into(),
                        )
                        .unwrap_or(::std::string::String::new())
                    }),
                cc: value.cc,
                name: ::std::string::String::from_utf8(value.name.to_vec())
                    .unwrap_or_else(|e| {
                        ::std::string::String::from_utf8(
                            value.name[..e.utf8_error().valid_up_to()].into(),
                        )
                        .unwrap_or(::std::string::String::new())
                    }),
                sessions: value.sessions.map(|x| x.try_into()?),
                mat: value.mat.map(|x| x.map(|x| x.try_into()?)),
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
            Ok(value) => Ok(value.try_into()?),
        }
    }
}
impl ::core::convert::TryFrom<&str> for User {
    type Error = ::pyo3::PyErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != <CUser>::SIZE * 2 {
            return Err(::pyo3::exceptions::PyValueError::new_err(
                "invalid length",
            ));
        }
        let value: Result<Vec<u8>, ::core::num::ParseIntError> = (0..value
            .len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
            .collect();
        let value = match value {
            Err(_) => {
                return Err(::pyo3::exceptions::PyValueError::new_err(
                    "invalid hex",
                ))
            }
            Ok(v) => v,
        };
        let value: CUser = value.as_slice().try_into()?;
        Ok(value.try_into()?)
    }
}
impl User {
    fn default() -> ::pyo3::PyResult<Self> {
        ::pyo3::Python::with_gil(|py| {
            Ok(Self {
                flag: 0,
                gene: ::pyo3::Py::new(py, <Gene>::default()?)?,
                agent: ::pyo3::Py::new(py, <Gene>::default()?)?,
                review: ::pyo3::Py::new(py, <Gene>::default()?)?,
                photo: ::pyo3::Py::new(py, <Gene>::default()?)?,
                phone: String::default(),
                cc: 0,
                name: String::default(),
                sessions: [
                    ::pyo3::Py::new(py, <Session>::default()?)?,
                    ::pyo3::Py::new(py, <Session>::default()?)?,
                    ::pyo3::Py::new(py, <Session>::default()?)?,
                ],
                mat: [
                    [
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                    ],
                    [
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                    ],
                    [
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                        ::pyo3::Py::new(py, <Gene>::default()?)?,
                    ],
                ],
            })
        })
    }
}
#[::pyo3::pymethods]
impl User {
    #[classattr]
    const SIZE: u64 = <CUser>::SIZE as u64;
    #[new]
    fn py_new(value: Option<&::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
        match value {
            Some(value) => {
                if let Ok(m) = value.extract::<User>() {
                    return Ok(m);
                }
                if let Ok(data) = value.extract::<&[u8]>() {
                    let m: Result<User, _> = data.try_into();
                    return match m {
                        Ok(m) => Ok(m),
                        Err(e) => {
                            Err(::pyo3::exceptions::PyValueError::new_err(e))
                        }
                    };
                }
                if let Ok(data) = value.extract::<String>() {
                    let m: Result<User, _> = data.as_str().try_into();
                    return match m {
                        Ok(m) => Ok(m),
                        Err(e) => {
                            Err(::pyo3::exceptions::PyValueError::new_err(e))
                        }
                    };
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
        let data: &[u8] = <CUser>::try_from(self.clone())?.into();
        Ok(data.to_owned().into())
    }
    fn __eq__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
        let a: &[u8] = <CUser>::try_from(self.clone())?.into();
        let b: &[u8] = <CUser>::try_from(other.clone())?.into();
        Ok(a == b)
    }
    #[setter]
    fn phone(&mut self, mut value: String) -> ::pyo3::PyResult<()> {
        let mut idx = 12usize;
        loop {
            if value.is_char_boundary(idx) {
                break;
            }
            idx -= 1;
        }
        value.truncate(idx);
        let value = match phone_validator(value) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        self.phone = value;
        Ok(())
    }
    #[setter]
    fn name(&mut self, mut value: String) -> ::pyo3::PyResult<()> {
        let mut idx = 50usize;
        loop {
            if value.is_char_boundary(idx) {
                break;
            }
            idx -= 1;
        }
        value.truncate(idx);
        self.name = value;
        Ok(())
    }
}
