use plutus_macros::model;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;




// use std::ptr::copy_nonoverlapping;
//
// fn string_to_array(s: String, len: usize) -> [u8] {
//     let data = s.as_bytes().to_vec();
//     data.resize(len, 0);
//
//     let mut buf: [u8; 10] = [1, 128, 235, 222, 12, 0, 122, 12, 4, 10];
//     println!(
//         "buf: {buf:?} {} {}: {}",
//         buf.len(),
//         g.len(),
//         g.len().min(buf.len())
//     );
//
//     // buf.copy_from_slice(&g[..10]);
//     unsafe {
//         copy_nonoverlapping(
//             s.as_ptr(),
//             buf.as_mut_ptr(),
//             s.len().min(buf.len()),
//         )
//     };
//
//     println!("s: {s:?}");
//     println!("g: {g:?}");
//     println!("buf: {buf:?}");
//     println!(
//         "new string: {:?}",
//         String::from_utf8(buf.into()).unwrap_or_else(|e| {
//             String::from_utf8(buf[..e.utf8_error().valid_up_to()].into())
//                 .unwrap_or(String::new())
//         }) // match String::from_utf8(buf.into()) {
//            //     Ok(b) => b,
//            //     Err(e) => {
//            //         let l = e.utf8_error().valid_up_to();
//            //         String::from_utf8(buf[..l].into()).unwrap()
//            //         // println!("{e:?}");
//            //         // "hi".to_owned()
//            //     }
//            // }
//     );
// }

fn phone_validator(value: String) -> PyResult<String> {
    let result = value.chars().enumerate().find_map(|(i, c)| {
        if c.is_ascii_digit() {
            return None;
        }

        Some(PyValueError::new_err(format!(
            "invalid phone number char: '{c}' at {i}"
        )))
    });

    match result {
        Some(err) => Err(err),
        None => Ok(value),
    }
}

#[model]
struct Gene {
    id: u32,
    pepper: u16,
    server: u16,
}

#[model(inner)]
struct SessionInfo {
    client: u8,
    os: u8,
    browser: u8,
    device: u8,
    client_version: u16,
    os_version: u16,
    browser_version: u16,
    _reserved: u16,
}

#[model(inner)]
struct Session {
    #[ipv4]
    ip: [u8; 4],
    info: SessionInfo,
    // if timestamp is 0, Session is Dead
    timestamp: u64,
    token: [u8; 64]
}

#[model]
struct User {
    flag: u64,
    gene: Gene,
    agent: Gene,
    review: Gene,
    photo: Gene,
    #[str(validator = phone_validator)]
    phone: [u8; 12],
    #[int(max=999)]
    cc: u16,
    #[str]
    name: [u8; 50],
    sessions: [Session; 3],
    mat: [[Gene; 3]; 3],
}

#[pymodule]
fn plutus(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // m.add_class::<SessionInfo>()?;
    // m.add_class::<Session>()?;
    m.add_class::<User>()?;
    m.add_class::<Gene>()?;
    Ok(())
}
