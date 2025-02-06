use ::libc;
#[inline]
unsafe extern "C" fn __gl_stdbit_clz(mut n: libc::c_uint) -> libc::c_int {
    return (if n != 0 {
        n.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    }) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __gl_stdbit_clzl(mut n: libc::c_ulong) -> libc::c_int {
    return (if n != 0 {
        n.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __gl_stdbit_clzll(mut n: libc::c_ulonglong) -> libc::c_int {
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
pub unsafe extern "C" fn stdc_leading_zeros_ui(mut n: libc::c_uint) -> libc::c_uint {
    return __gl_stdbit_clz(n) as libc::c_uint;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stdc_leading_zeros_uc(mut n: libc::c_uchar) -> libc::c_uint {
    return (stdc_leading_zeros_ui(n as libc::c_uint) as libc::c_ulong)
        .wrapping_sub(
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_sub(
                            ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                ),
        ) as libc::c_uint;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stdc_leading_zeros_us(mut n: libc::c_ushort) -> libc::c_uint {
    return (stdc_leading_zeros_ui(n as libc::c_uint) as libc::c_ulong)
        .wrapping_sub(
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_sub(
                            ::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                        ),
                ),
        ) as libc::c_uint;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stdc_leading_zeros_ul(mut n: libc::c_ulong) -> libc::c_uint {
    return __gl_stdbit_clzl(n) as libc::c_uint;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stdc_leading_zeros_ull(
    mut n: libc::c_ulonglong,
) -> libc::c_uint {
    return __gl_stdbit_clzll(n) as libc::c_uint;
}
