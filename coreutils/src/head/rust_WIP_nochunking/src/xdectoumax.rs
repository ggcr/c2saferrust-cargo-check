
use ::libc;
extern "C" {
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub const LONGINT_OK: strtol_error = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
#[no_mangle]
pub unsafe extern "C" fn xdectoumax(
    mut n_str: *const libc::c_char,
    mut min: uintmax_t,
    mut max: uintmax_t,
    mut suffixes: *const libc::c_char,
    mut err: *const libc::c_char,
    mut err_exit: libc::c_int,
) -> uintmax_t {
    let result = xnumtoumax(
    std::ffi::CStr::from_ptr(n_str).to_str().unwrap(),
    10,
    min,
    max,
    suffixes,
    err,
    err_exit,
);
return result;
}
#[no_mangle]
pub fn xnumtoumax(
    n_str: &str,
    base: i32,
    min: u64,
    max: u64,
    suffixes: *const libc::c_char,
    err: *const libc::c_char,
    err_exit: i32,
) -> u64 {
    let mut tnum: u64 = 0;
    let mut end_ptr: *mut libc::c_char = std::ptr::null_mut();
    
    unsafe {
        let s_err = xstrtoumax(n_str.as_ptr() as *const libc::c_char, &mut end_ptr, base, &mut tnum, suffixes);

        match s_err {
            LONGINT_OK => {
                if tnum < min || tnum > max {
                    let exit_code = if err_exit != 0 { err_exit } else { 1 };
                    error(exit_code, 34, b"Value out of range: %s\0".as_ptr() as *const i8, quote(n_str.as_ptr() as *const libc::c_char));
                    unreachable!();
                }
            }
            LONGINT_OVERFLOW => {
                let exit_code = if err_exit != 0 { err_exit } else { 1 };
                error(exit_code, 75, b"Overflow occurred\0".as_ptr() as *const i8);
                unreachable!();
            }
            LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW => {
                let exit_code = if err_exit != 0 { err_exit } else { 1 };
                error(exit_code, 0, b"Invalid suffix character with overflow\0".as_ptr() as *const i8);
                unreachable!();
            }
            _ => {
                let exit_code = if err_exit != 0 { err_exit } else { 1 };
                error(exit_code, 1, b"Unknown error\0".as_ptr() as *const i8);
                unreachable!();
            }
        }
    }

    tnum
}

