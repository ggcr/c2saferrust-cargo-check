

use std::convert::TryInto;

use std::ffi::CStr;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn usage(_e: libc::c_int);
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
extern "C" fn __argmatch_die() {
    unsafe {
        usage(1);
    }
}

#[no_mangle]
pub static mut argmatch_die: argmatch_exit_fn = unsafe {
    Some(__argmatch_die as unsafe extern "C" fn() -> ())
};
#[no_mangle]
pub unsafe extern "C" fn argmatch(
    mut arg: *const libc::c_char,
    mut arglist: *const *const libc::c_char,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) -> ptrdiff_t {
    let mut i: size_t = 0;
    let mut arglen: size_t = 0;
    let mut matchind: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
    let mut ambiguous: bool = 0 as libc::c_int != 0;
    arglen = strlen(arg);
    i = 0 as libc::c_int as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if strncmp(*arglist.offset(i as isize), arg, arglen) == 0 {
            if strlen(*arglist.offset(i as isize)) == arglen {
                return i as ptrdiff_t
            } else if matchind == -(1 as libc::c_int) as libc::c_long {
                matchind = i as ptrdiff_t;
            } else if vallist.is_null()
                || memcmp(
                    (vallist as *const libc::c_char)
                        .offset(valsize.wrapping_mul(matchind as libc::c_ulong) as isize)
                        as *const libc::c_void,
                    (vallist as *const libc::c_char)
                        .offset(valsize.wrapping_mul(i) as isize) as *const libc::c_void,
                    valsize,
                ) != 0
            {
                ambiguous = 1 as libc::c_int != 0;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if ambiguous { return -(2 as libc::c_int) as ptrdiff_t } else { return matchind };
}
#[no_mangle]
pub fn argmatch_exact(
    arg: &str,
    arglist: &[&str],
) -> isize {
    for (i, &item) in arglist.iter().enumerate() {
        if item == arg {
            return i as isize;
        }
    }
    return -1;
}

#[no_mangle]
pub fn argmatch_invalid(
    context: &CStr,
    value: &CStr,
    problem: isize,
) {
    let format = if problem == -1 {
        unsafe { CStr::from_ptr(gettext(b"invalid argument %s for %s\0".as_ptr() as *const i8)) }
    } else {
        unsafe { CStr::from_ptr(gettext(b"ambiguous argument %s for %s\0".as_ptr() as *const i8)) }
    };

    let quoted_value = unsafe { quotearg_n_style(0, locale_quoting_style, value.as_ptr()) };
    let quoted_context = unsafe { quote_n(1, context.as_ptr()) };

    let errstatus: i32 = 0;
    unsafe {
        error(
            errstatus,
            0,
            format.as_ptr(),
            quoted_value,
            quoted_context,
        );
    }

    if errstatus != 0 {
        unreachable!();
    }
}

#[no_mangle]
pub unsafe extern "C" fn argmatch_valid(
    mut arglist: *const *const libc::c_char,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) {
    let mut i: size_t = 0;
    let mut last_val: *const libc::c_char = 0 as *const libc::c_char;
    fputs_unlocked(
        gettext(b"Valid arguments are:\0" as *const u8 as *const libc::c_char),
        stderr,
    );
    i = 0 as libc::c_int as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if i == 0 as libc::c_int as libc::c_ulong
            || memcmp(
                last_val as *const libc::c_void,
                (vallist as *const libc::c_char).offset(valsize.wrapping_mul(i) as isize)
                    as *const libc::c_void,
                valsize,
            ) != 0
        {
            fprintf(
                stderr,
                b"\n  - %s\0" as *const u8 as *const libc::c_char,
                quote(*arglist.offset(i as isize)),
            );
            last_val = (vallist as *const libc::c_char)
                .offset(valsize.wrapping_mul(i) as isize);
        } else {
            fprintf(
                stderr,
                b", %s\0" as *const u8 as *const libc::c_char,
                quote(*arglist.offset(i as isize)),
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    putc_unlocked('\n' as i32, stderr);
}
#[no_mangle]
pub fn __xargmatch_internal(
    context: &CStr,
    arg: &CStr,
    arglist: &[&CStr],
    vallist: *const libc::c_void,
    valsize: usize,
    exit_fn: argmatch_exit_fn,
    allow_abbreviation: bool,
) -> isize {
    let res: isize;
    if allow_abbreviation {
        unsafe {
            res = argmatch(arg.as_ptr(), arglist.as_ptr() as *const *const libc::c_char, vallist, valsize.try_into().unwrap()) as isize;
        }
    } else {
        unsafe {
            res = argmatch_exact(arg.to_str().unwrap(), &arglist.iter().map(|s| s.to_str().unwrap()).collect::<Vec<_>>()) as isize;
        }
    }
    if res >= 0 {
        return res;
    }
    unsafe {
        argmatch_invalid(context, arg, res);
        argmatch_valid(arglist.as_ptr() as *const *const libc::c_char, vallist, valsize.try_into().unwrap());
        if let Some(exit_fn) = exit_fn {
            exit_fn();
        }
    }
    return -1;
}

#[no_mangle]
pub unsafe extern "C" fn argmatch_to_argument(
    mut value: *const libc::c_void,
    mut arglist: *const *const libc::c_char,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) -> *const libc::c_char {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if memcmp(
            value,
            (vallist as *const libc::c_char).offset(valsize.wrapping_mul(i) as isize)
                as *const libc::c_void,
            valsize,
        ) == 0
        {
            return *arglist.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const libc::c_char;
}
