

use std::ffi::CStr;

use std::convert::TryFrom;

use std::slice;

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
pub unsafe extern "C" fn skip_str_matching(
    mut str: *const libc::c_char,
    mut predicate: Option::<unsafe extern "C" fn(mcel_t) -> bool>,
    mut ok: bool,
) -> *mut libc::c_char {
    let mut s: *const libc::c_char = str;
    let mut g: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };
    while *s as libc::c_int != 0
        && {
            g = mcel_scanz(s);
            predicate.expect("non-null function pointer")(g) as libc::c_int
                == ok as libc::c_int
        }
    {
        s = s.offset(g.len as libc::c_int as isize);
    }
    return s as *mut libc::c_char;
}
#[inline]
fn mcel_scanz(p: *const libc::c_char) -> mcel_t {
    let c_str = unsafe { CStr::from_ptr(p) };
    let str_slice = c_str.to_str().unwrap(); // Handle potential UTF-8 errors as needed
    let result = unsafe { mcel_scant(CStr::from_ptr(str_slice.as_ptr() as *const libc::c_char), '\0') };
return result;
}

#[inline]
fn mcel_scant(p: &CStr, terminator: char) -> mcel_t {
    let p_bytes = p.to_bytes();
    if mcel_isbasic(p_bytes[0] as libc::c_char) {
        return unsafe { mcel_ch(p_bytes[0] as char32_t, 1) };
    }
    
    let mut lim = &p_bytes[1..];
    let mut i = 0;
    while i < MCEL_LEN_MAX as libc::c_int - 1 {
        if lim.is_empty() || lim[0] as libc::c_int == terminator as libc::c_int {
            break;
        }
        lim = &lim[1..];
        i += 1;
    }
    return unsafe { mcel_scan(p.as_ptr() as *const libc::c_char, lim.as_ptr() as *const libc::c_char) };
}

#[inline]
fn mcel_scan(p: *const libc::c_char, lim: *const libc::c_char) -> mcel_t {
    let c = unsafe { *p };
    if unsafe { mcel_isbasic(c) } {
        return unsafe { mcel_ch(c as char32_t, 1) };
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
            (lim as usize).wrapping_sub(p as usize) as u64,
            &mut mbs,
        )
    };
    
    if len > (-(1 as libc::c_int) as u64) / 2 {
        return unsafe { mcel_err(c as libc::c_uchar) };
    }
    
    return unsafe { mcel_ch(ch, len) };
}

#[inline]
fn mcel_isbasic(c: i8) -> bool {
    (0 <= c as i32 && (c as i32) < MCEL_ERR_MIN as i32)
}

#[inline]
unsafe extern "C" fn mcel_err(mut err: libc::c_uchar) -> mcel_t {
    if MCEL_ERR_MIN as libc::c_int <= err as libc::c_int {} else {
        unreachable!();
    };
    return {
        let mut init = mcel_t {
            ch: 0,
            err: err,
            len: 1 as libc::c_int as libc::c_uchar,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn mcel_ch(mut ch: char32_t, mut len: size_t) -> mcel_t {
    if (0 as libc::c_int as libc::c_ulong) < len {} else {
        unreachable!();
    };
    if len <= MCEL_LEN_MAX as libc::c_int as libc::c_ulong {} else {
        unreachable!();
    };
    if ch <= MCEL_CHAR_MAX as libc::c_int as libc::c_uint {} else {
        unreachable!();
    };
    return {
        let mut init = mcel_t {
            ch: ch,
            err: 0,
            len: len as libc::c_uchar,
        };
        init
    };
}
