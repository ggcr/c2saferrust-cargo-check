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
pub unsafe extern "C" fn mcel_scanz(mut p: *const libc::c_char) -> mcel_t {
    return mcel_scant(p, '\0' as i32 as libc::c_char);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mcel_scant(
    mut p: *const libc::c_char,
    mut terminator: libc::c_char,
) -> mcel_t {
    if mcel_isbasic(*p) {
        return mcel_ch(*p as char32_t, 1 as libc::c_int as size_t);
    }
    let mut lim: *const libc::c_char = p.offset(1 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < MCEL_LEN_MAX as libc::c_int - 1 as libc::c_int {
        lim = lim
            .offset(
                (*lim as libc::c_int != terminator as libc::c_int) as libc::c_int
                    as isize,
            );
        i += 1;
        i;
    }
    return mcel_scan(p, lim);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mcel_scan(
    mut p: *const libc::c_char,
    mut lim: *const libc::c_char,
) -> mcel_t {
    let mut c: libc::c_char = *p;
    if mcel_isbasic(c) {
        return mcel_ch(c as char32_t, 1 as libc::c_int as size_t);
    }
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    mbs.__count = 0 as libc::c_int;
    let mut ch: char32_t = 0;
    let mut len: size_t = mbrtoc32(
        &mut ch,
        p,
        lim.offset_from(p) as libc::c_long as size_t,
        &mut mbs,
    );
    if ((-(1 as libc::c_int) as size_t).wrapping_div(2 as libc::c_int as libc::c_ulong)
        < len) as libc::c_int as libc::c_long != 0
    {
        return mcel_err(c as libc::c_uchar);
    }
    return mcel_ch(ch, len);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mcel_isbasic(mut c: libc::c_char) -> bool {
    return (0 as libc::c_int <= c as libc::c_int
        && (c as libc::c_int) < MCEL_ERR_MIN as libc::c_int) as libc::c_int
        as libc::c_long != 0;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mcel_tocmp(
    mut to: Option::<unsafe extern "C" fn(wint_t) -> wint_t>,
    mut c1: mcel_t,
    mut c2: mcel_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = mcel_cmp(c1, c2);
    if (c1.err as libc::c_int - c2.err as libc::c_int | (cmp == 0) as libc::c_int)
        as libc::c_long != 0
    {
        return cmp;
    }
    let mut ch1: libc::c_int = to.expect("non-null function pointer")(c1.ch)
        as libc::c_int;
    let mut ch2: libc::c_int = to.expect("non-null function pointer")(c2.ch)
        as libc::c_int;
    return ch1 - ch2;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mcel_cmp(mut c1: mcel_t, mut c2: mcel_t) -> libc::c_int {
    let mut ch1: libc::c_int = c1.ch as libc::c_int;
    let mut ch2: libc::c_int = c2.ch as libc::c_int;
    return (c1.err as libc::c_int - c2.err as libc::c_int)
        * ((1 as libc::c_int) << MCEL_ERR_SHIFT as libc::c_int) + (ch1 - ch2);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mcel_err(mut err: libc::c_uchar) -> mcel_t {
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mcel_ch(mut ch: char32_t, mut len: size_t) -> mcel_t {
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
