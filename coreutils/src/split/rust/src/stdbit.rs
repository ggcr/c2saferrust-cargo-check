use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_stdbit_clz(mut n: libc::c_uint) -> libc::c_int {
    return (if n != 0 {
        n.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    }) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_stdbit_clzl(mut n: libc::c_ulong) -> libc::c_int {
    return (if n != 0 {
        n.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_stdbit_clzll(mut n: libc::c_ulonglong) -> libc::c_int {
    return (if n != 0 {
        n.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_stdbit_ctz(mut n: libc::c_uint) -> libc::c_int {
    return (if n != 0 {
        n.trailing_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    }) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_stdbit_ctzl(mut n: libc::c_ulong) -> libc::c_int {
    return (if n != 0 {
        n.trailing_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_stdbit_ctzll(mut n: libc::c_ulonglong) -> libc::c_int {
    return (if n != 0 {
        n.trailing_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
    }) as libc::c_int;
}
