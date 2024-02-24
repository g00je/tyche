use std::ptr::copy_nonoverlapping;
// mod out;

fn main() {
    let s = "some str     ".to_string();
    let g = s.as_bytes();

    let mut buf: [u8; 10] = [1, 128, 235, 222, 12, 0, 122, 12, 4, 10];
    println!(
        "buf: {buf:?} {} {}: {}",
        buf.len(),
        g.len(),
        g.len().min(buf.len())
    );

    // buf.copy_from_slice(&g[..10]);
    unsafe {
        copy_nonoverlapping(
            s.as_ptr(),
            buf.as_mut_ptr(),
            s.len().min(buf.len()),
        )
    };

    println!("s: {s:?}");
    println!("g: {g:?}");
    println!("buf: {buf:?}");
    println!(
        "new string: {:?}",
        String::from_utf8(buf.into()).unwrap_or_else(|e| {
            String::from_utf8(buf[..e.utf8_error().valid_up_to()].into())
                .unwrap_or(String::new())
        }) 
        // match String::from_utf8(buf.into()) {
           //     Ok(b) => b,
           //     Err(e) => {
           //         let l = e.utf8_error().valid_up_to();
           //         String::from_utf8(buf[..l].into()).unwrap()
           //         // println!("{e:?}");
           //         // "hi".to_owned()
           //     }
           // }
    );
}
