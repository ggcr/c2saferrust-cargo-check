






use std::ffi::CString;

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
pub type wint_t = libc::c_uint;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MCEL_ERR_SHIFT: C2RustUnnamed_3 = 14;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_scanz(p: &str) -> mcel_t {
    let c_str = std::ffi::CString::new(p).unwrap();
    mcel_scant(c_str.as_ptr(), '\0' as libc::c_char)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_scant(p: *const libc::c_char, terminator: libc::c_char) -> mcel_t {
    let p_str = unsafe { std::ffi::CStr::from_ptr(p).to_string_lossy().into_owned() };
    
    let first_char = p_str.chars().next().unwrap() as libc::c_char;
    if unsafe { mcel_isbasic(first_char) } {
        return mcel_ch(first_char as char32_t, 1);
    }
    
    let mut lim = &p_str[1..];
    let mut i = 0;

    while i < MCEL_LEN_MAX - 1 {
        if lim.chars().next().unwrap() != terminator as u8 as char {
            lim = &lim[1..];
        }
        i += 1;
    }
    
    return mcel_scan(&p_str, lim);
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_scan(p: &str, lim: &str) -> mcel_t {
    let c = p.chars().next().unwrap_or('\0') as libc::c_char;
    if unsafe { mcel_isbasic(c) } {
        return mcel_ch(c as char32_t, 1);
    }
    
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    mbs.__count = 0;
    let mut ch: char32_t = 0;
    
    let len = unsafe {
        mbrtoc32(
            &mut ch,
            p.as_ptr() as *const libc::c_char,
            (lim.as_ptr().offset_from(p.as_ptr()) as usize) as u64,
            &mut mbs,
        )
    };
    
    if len == u64::MAX {
        return mcel_err(c as libc::c_uchar);
    }
    
    return mcel_ch(ch, len as usize);
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_isbasic(c: i8) -> bool {
    (0 <= c as i32 && (c as i32) < MCEL_ERR_MIN as i32)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_tocmp(
    to: Option<fn(wint_t) -> wint_t>,
    c1: mcel_t,
    c2: mcel_t,
) -> libc::c_int {
    let cmp: libc::c_int = mcel_cmp(c1, c2);
    if (c1.err as libc::c_int - c2.err as libc::c_int | (cmp == 0) as libc::c_int) != 0 {
        return cmp;
    }
    let ch1: libc::c_int = to.expect("non-null function pointer")(c1.ch).try_into().unwrap();
    let ch2: libc::c_int = to.expect("non-null function pointer")(c2.ch).try_into().unwrap();
    ch1 - ch2
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_cmp(c1: mcel_t, c2: mcel_t) -> i32 {
    let ch1: i32 = c1.ch as i32;
    let ch2: i32 = c2.ch as i32;
    (c1.err as i32 - c2.err as i32) * (1 << MCEL_ERR_SHIFT) + (ch1 - ch2)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_err(err: u8) -> mcel_t {
    assert!(MCEL_ERR_MIN as i32 <= err as i32, "Error value out of range");
    
    mcel_t {
        ch: 0,
        err,
        len: 1,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_ch(ch: char32_t, len: usize) -> mcel_t {
    assert!(len > 0, "Length must be greater than 0");
    assert!(len <= MCEL_LEN_MAX as usize, "Length exceeds maximum allowed");
    assert!(ch <= MCEL_CHAR_MAX as u32, "Character exceeds maximum allowed");

    mcel_t {
        ch,
        err: 0,
        len: len as u8,
    }
}

