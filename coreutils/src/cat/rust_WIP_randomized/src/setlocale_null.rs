
use std::ffi::CStr;

use ::libc;
extern "C" {
    fn setlocale_null_r_unlocked(
        category: libc::c_int,
        buf: *mut libc::c_char,
        bufsize: size_t,
    ) -> libc::c_int;
    fn setlocale_null_unlocked(category: libc::c_int) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub fn setlocale_null_r(
    category: libc::c_int,
    buf: &mut [u8],
) -> libc::c_int {
    let bufsize = buf.len() as u64;
    let buf_ptr = buf.as_mut_ptr() as *mut libc::c_char;
    unsafe {
        return setlocale_null_r_unlocked(category, buf_ptr, bufsize);
    }
}

#[no_mangle]
pub fn setlocale_null(category: libc::c_int) -> Option<String> {
    unsafe {
        let result = setlocale_null_unlocked(category);
        if result.is_null() {
            None
        } else {
            // Assuming the result is a null-terminated C string, we convert it to a Rust String
            let c_str = std::ffi::CStr::from_ptr(result);
            c_str.to_str().ok().map(|s| s.to_string())
        }
    }
}

