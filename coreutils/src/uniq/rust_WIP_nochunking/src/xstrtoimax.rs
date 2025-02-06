

use ::libc;
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoimax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> intmax_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __intmax_t = libc::c_long;
pub type intmax_t = __intmax_t;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub const _ISspace: C2RustUnnamed = 8192;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
fn bkm_scale(x: &mut i64, scale_factor: i32) -> strtol_error {
    let mut scaled: i64 = 0;

    if (0 < -1) && (x.wrapping_sub(1) < 0) && (scale_factor - 1 < 0) {
        if scale_factor < 0 {
            if *x < 0 {
                if (0 < -1 / scale_factor as i64) && (*x < -(1 / scale_factor as i64)) {
                    return LONGINT_OVERFLOW;
                }
            } else {
                if (scale_factor < 0) && (scale_factor < -1) {
                    return LONGINT_OVERFLOW;
                }
            }
        } else {
            if scale_factor == 0 {
                return LONGINT_OK;
            } else {
                if *x < 0 {
                    if (0 < *x) {
                        return LONGINT_OVERFLOW;
                    }
                } else {
                    if (-(1 / scale_factor as i64) < *x) {
                        return LONGINT_OVERFLOW;
                    }
                }
            }
        }
    }

    let (result, overflow) = x.overflowing_mul(scale_factor as i64);
    *x = if overflow { 
        if *x < 0 {
            i64::MIN
        } else {
            i64::MAX
        }
    } else {
        result
    };

    if overflow {
        return LONGINT_OVERFLOW;
    }
    LONGINT_OK
}

fn bkm_scale_by_power(
    x: &mut i64, // Assuming intmax_t is equivalent to i64 in Rust
    base: i32,
    mut power: i32,
) -> strtol_error {
    let mut err: strtol_error = LONGINT_OK;
    while power > 0 {
        power -= 1;
        err |= bkm_scale(x, base);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn xstrtoimax(
    mut s: *const libc::c_char,
    mut ptr: *mut *mut libc::c_char,
    mut strtol_base: libc::c_int,
    mut val: *mut intmax_t,
    mut valid_suffixes: *const libc::c_char,
) -> strtol_error {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: intmax_t = 0;
    let mut err: strtol_error = LONGINT_OK;
    if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
        __assert_fail(
            b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                as *const libc::c_char,
            b"./include/xstrtol.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"strtol_error xstrtoimax(const char *, char **, int, intmax_t *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2247: {
        if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
            __assert_fail(
                b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                    as *const libc::c_char,
                b"./include/xstrtol.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"strtol_error xstrtoimax(const char *, char **, int, intmax_t *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    p = if !ptr.is_null() { ptr } else { &mut t_ptr };
    *__errno_location() = 0 as libc::c_int;
    if (0 as libc::c_int as intmax_t) < -(1 as libc::c_int) as intmax_t {
        let mut q: *const libc::c_char = s;
        let mut ch: libc::c_uchar = *q as libc::c_uchar;
        while *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            q = q.offset(1);
            ch = *q as libc::c_uchar;
        }
        if ch as libc::c_int == '-' as i32 {
            return LONGINT_INVALID;
        }
    }
    tmp = strtoimax(s, p, strtol_base);
    if *p == s as *mut libc::c_char {
        if !valid_suffixes.is_null() && **p as libc::c_int != 0
            && !(strchr(valid_suffixes, **p as libc::c_int)).is_null()
        {
            tmp = 1 as libc::c_int as intmax_t;
        } else {
            return LONGINT_INVALID
        }
    } else if *__errno_location() != 0 as libc::c_int {
        if *__errno_location() != 34 as libc::c_int {
            return LONGINT_INVALID;
        }
        err = LONGINT_OVERFLOW;
    }
    if valid_suffixes.is_null() {
        *val = tmp;
        return err;
    }
    if **p as libc::c_int != '\0' as i32 {
        let mut base: libc::c_int = 1024 as libc::c_int;
        let mut suffixes: libc::c_int = 1 as libc::c_int;
        let mut overflow: strtol_error = LONGINT_OK;
        if (strchr(valid_suffixes, **p as libc::c_int)).is_null() {
            *val = tmp;
            return (err as libc::c_uint
                | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint)
                as strtol_error;
        }
        match **p as libc::c_int {
            69 | 71 | 103 | 107 | 75 | 77 | 109 | 80 | 81 | 82 | 84 | 116 | 89 | 90 => {
                if !(strchr(valid_suffixes, '0' as i32)).is_null() {
                    match *(*p.offset(0 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int
                    {
                        105 => {
                            if *(*p.offset(0 as libc::c_int as isize))
                                .offset(2 as libc::c_int as isize) as libc::c_int
                                == 'B' as i32
                            {
                                suffixes += 2 as libc::c_int;
                            }
                        }
                        66 | 68 => {
                            base = 1000 as libc::c_int;
                            suffixes += 1;
                            suffixes;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        match **p as libc::c_int {
            98 => {
                let overflow = bkm_scale(&mut tmp, 512);
            }
            66 => {
                let overflow = bkm_scale(&mut tmp, 1024);
            }
            99 => {
                overflow = LONGINT_OK;
            }
            69 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 6);
            }
            71 | 103 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 3);
            }
            107 | 75 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 1);
            }
            77 | 109 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 2);
            }
            80 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 5);
            }
            81 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 10);
            }
            82 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 9);
            }
            84 | 116 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 4);
            }
            119 => {
                let overflow = bkm_scale(&mut tmp, 2);
            }
            89 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 8);
            }
            90 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 7);
            }
            _ => {
                *val = tmp;
                return (err as libc::c_uint
                    | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint)
                    as strtol_error;
            }
        }
        err = ::core::mem::transmute::<
            libc::c_uint,
            strtol_error,
        >(err as libc::c_uint | overflow as libc::c_uint);
        *p = (*p).offset(suffixes as isize);
        if **p != 0 {
            err = ::core::mem::transmute::<
                libc::c_uint,
                strtol_error,
            >(
                err as libc::c_uint
                    | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint,
            );
        }
    }
    *val = tmp;
    return err;
}
