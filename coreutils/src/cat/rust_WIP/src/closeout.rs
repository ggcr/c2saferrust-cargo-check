
use std::ffi::CString;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn _exit(_: libc::c_int) -> !;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn close_stream(stream: *mut FILE) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub const SANITIZE_ADDRESS: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
static mut file_name: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub fn close_stdout_set_file_name(file: &str) {
    let c_string = std::ffi::CString::new(file).unwrap();
    unsafe {
        file_name = c_string.as_ptr();
        std::mem::forget(c_string); // Prevent CString from being dropped
    }
}

static mut ignore_EPIPE: bool = false;
#[no_mangle]
pub fn close_stdout_set_ignore_EPIPE(ignore: bool) {
    unsafe {
        ignore_EPIPE = ignore;
    }
}

#[no_mangle]
pub unsafe extern "C" fn close_stdout() {
    if close_stream(stdout) != 0 as libc::c_int
        && !(ignore_EPIPE as libc::c_int != 0
            && *__errno_location() == 32 as libc::c_int)
    {
        let mut write_error: *const libc::c_char = gettext(
            b"write error\0" as *const u8 as *const libc::c_char,
        );
        if !file_name.is_null() {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    quotearg_colon(file_name),
                    write_error,
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        quotearg_colon(file_name),
                        write_error,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        quotearg_colon(file_name),
                        write_error,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        } else {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    write_error,
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        write_error,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        write_error,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        _exit(exit_failure);
    }
    if SANITIZE_ADDRESS as libc::c_int == 0 && close_stream(stderr) != 0 as libc::c_int {
        _exit(exit_failure);
    }
}
