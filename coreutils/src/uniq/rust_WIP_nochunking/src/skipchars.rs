





use std::slice;

use std::ffi::CStr;

use ::libc;
extern "C" {
    fn mbrtoc32(
        __pc32: *mut char32_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint_least32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type mbstate_t = __mbstate_t;
pub type char32_t = __uint_least32_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MCEL_LEN_MAX: C2RustUnnamed_0 = 4;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MCEL_CHAR_MAX: C2RustUnnamed_1 = 1114111;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MCEL_ERR_MIN: C2RustUnnamed_2 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcel_t {
    pub ch: char32_t,
    pub err: libc::c_uchar,
    pub len: libc::c_uchar,
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn skip_buf_matching<'a>(
    buf: &'a [libc::c_char],
    lim: &'a [libc::c_char],
    predicate: Option<fn(mcel_t) -> bool>,
    ok: bool,
) -> &'a [libc::c_char] {
    let mut s = buf.as_ptr();
    let lim_ptr = lim.as_ptr();
    let mut g: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };

    while s < lim_ptr && {
        g = mcel_scan(s, lim_ptr);
        predicate.expect("non-null function pointer")(g) == ok
    } {
        s = unsafe { s.add(g.len as usize) };
    }

    unsafe { std::slice::from_raw_parts(s, lim.len() - (s as usize - buf.as_ptr() as usize)) }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn skip_str_matching(
    str: &CStr,
    predicate: Option<fn(mcel_t) -> bool>,
    ok: bool,
) -> &CStr {
    let mut s = str.as_ptr();
    let mut g: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };
    let mut bytes = str.to_bytes();

    while !bytes.is_empty() && {
        g = mcel_scanz(s);
        predicate.expect("non-null function pointer")(g) == ok
    } {
        let len = g.len as usize;
        bytes = &bytes[len..];
        s = bytes.as_ptr() as *const libc::c_char;
    }
    
    unsafe { CStr::from_ptr(s) }
}

#[inline]
fn mcel_scanz(p: *const libc::c_char) -> mcel_t {
    let c_str = unsafe { CStr::from_ptr(p) };
    let str_slice = c_str.to_str().unwrap(); // Handle potential errors as needed
    mcel_scant(str_slice, '\0')
}

#[inline]
fn mcel_scant(p: &str, terminator: char) -> mcel_t {
    if mcel_isbasic(p.chars().next().unwrap() as i8) {
        return mcel_ch(p.chars().next().unwrap() as char32_t, 1);
    }
    
    let mut lim = &p[1..];
    let mut i = 0;
    
    while i < MCEL_LEN_MAX - 1 {
        if lim.chars().next().unwrap() == terminator {
            break;
        }
        lim = &lim[1..];
        i += 1;
    }
    
    return mcel_scan(p.as_ptr() as *const libc::c_char, lim.as_ptr() as *const libc::c_char);
}

#[inline]
fn mcel_scan(p: *const libc::c_char, lim: *const libc::c_char) -> mcel_t {
    let c = unsafe { *p };
    if mcel_isbasic(c as i8) {
        return mcel_ch(c as char32_t, 1);
    }
    
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    
    let mut ch: char32_t = 0;
    let len = unsafe {
        mbrtoc32(
            &mut ch,
            p,
            lim.offset_from(p) as u64,
            &mut mbs,
        )
    };
    
    if len > (-(1 as libc::c_int) as u64) / 2 {
        return mcel_err(c as libc::c_uchar);
    }
    
    return mcel_ch(ch, len as usize);
}

#[inline]
fn mcel_isbasic(c: i8) -> bool {
    (0 <= c as i32 && (c as i32) < MCEL_ERR_MIN as i32)
}

#[inline]
fn mcel_err(err: u8) -> mcel_t {
    assert!(MCEL_ERR_MIN as i32 <= err as i32);
    
    mcel_t {
        ch: 0,
        err,
        len: 1,
    }
}

#[inline]
fn mcel_ch(ch: char32_t, len: usize) -> mcel_t {
    assert!(len > 0, "Length must be greater than 0");
    assert!(len <= MCEL_LEN_MAX as usize, "Length exceeds maximum allowed");
    assert!(ch <= MCEL_CHAR_MAX as u32, "Character exceeds maximum allowed");

    mcel_t {
        ch,
        err: 0,
        len: len as u8,
    }
}

