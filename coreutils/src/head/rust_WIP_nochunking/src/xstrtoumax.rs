
use ::libc;
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> uintmax_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
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
unsafe extern "C" fn bkm_scale(
    mut x: *mut uintmax_t,
    mut scale_factor: libc::c_int,
) -> strtol_error {
    let mut scaled: uintmax_t = 0;
    if if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t
        && (if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_ulong } else { *x })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < 0 as libc::c_int as libc::c_ulong
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
            - 1 as libc::c_int) < 0 as libc::c_int
        && (if scale_factor < 0 as libc::c_int {
            if *x < 0 as libc::c_int as libc::c_ulong {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        -(1 as libc::c_int) as uintmax_t
                    })
                        .wrapping_add(scale_factor as libc::c_ulong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    (*x
                        < (-(1 as libc::c_int) as uintmax_t)
                            .wrapping_div(scale_factor as libc::c_ulong)) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (scale_factor
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < scale_factor) as libc::c_int
                    }) != 0
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_ulong)
                            .wrapping_add(-(1 as libc::c_int) as uintmax_t)
                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (-(1 as libc::c_int) as uintmax_t)
                            .wrapping_div(-scale_factor as libc::c_ulong)
                    }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(*x))
                        as libc::c_int
                }
            } else {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as uintmax_t)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    !((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_ulong)
                            .wrapping_add(0 as libc::c_int as uintmax_t)
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_ulong)
                            .wrapping_add(0 as libc::c_int as uintmax_t)
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                }) < 0 as libc::c_int as libc::c_ulong
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as uintmax_t)
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) as libc::c_ulong)
                                .wrapping_add(0 as libc::c_int as uintmax_t)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as uintmax_t)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as uintmax_t)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_neg()) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong)
                        < ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_ulong)
                            .wrapping_add(0 as libc::c_int as uintmax_t)) as libc::c_int
                }) != 0 && scale_factor == -(1 as libc::c_int)
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        *x
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (*x).wrapping_add(0 as libc::c_int as uintmax_t))
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < *x
                            && (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub(0 as libc::c_int as uintmax_t)
                                < (*x).wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    ((0 as libc::c_int as uintmax_t)
                        .wrapping_div(scale_factor as libc::c_ulong) < *x) as libc::c_int
                }
            }
        } else {
            if scale_factor == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                if *x < 0 as libc::c_int as libc::c_ulong {
                    if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            *x
                        })
                            .wrapping_add(0 as libc::c_int as uintmax_t)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                *x
                            })
                                .wrapping_add(0 as libc::c_int as uintmax_t)
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                *x
                            })
                                .wrapping_add(0 as libc::c_int as uintmax_t)
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            *x
                        })
                            .wrapping_add(0 as libc::c_int as uintmax_t)
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    *x
                                })
                                    .wrapping_add(0 as libc::c_int as uintmax_t)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        *x
                                    })
                                        .wrapping_add(0 as libc::c_int as uintmax_t)
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        *x
                                    })
                                        .wrapping_add(0 as libc::c_int as uintmax_t)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                *x
                            })
                                .wrapping_add(0 as libc::c_int as uintmax_t)) as libc::c_int
                    }) != 0 && *x == -(1 as libc::c_int) as libc::c_ulong
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < (scale_factor as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as uintmax_t)) as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub(0 as libc::c_int as uintmax_t)
                                < (scale_factor - 1 as libc::c_int) as libc::c_ulong)
                                as libc::c_int
                        }
                    } else {
                        ((0 as libc::c_int as uintmax_t).wrapping_div(*x)
                            < scale_factor as libc::c_ulong) as libc::c_int
                    }
                } else {
                    ((-(1 as libc::c_int) as uintmax_t)
                        .wrapping_div(scale_factor as libc::c_ulong) < *x) as libc::c_int
                }
            }
        }) != 0
    {
        let (fresh4, _fresh5) = (*x).overflowing_mul(scale_factor.try_into().unwrap());
        *(&mut scaled as *mut uintmax_t) = fresh4;
        1 as libc::c_int
    } else {
        let (fresh6, fresh7) = (*x).overflowing_mul(scale_factor.try_into().unwrap());
        *(&mut scaled as *mut uintmax_t) = fresh6;
        fresh7 as libc::c_int
    } != 0
    {
        *x = if *x < 0 as libc::c_int as libc::c_ulong {
            !if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t {
                -(1 as libc::c_int) as uintmax_t
            } else {
                ((1 as libc::c_int as uintmax_t)
                    << (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            }
        } else if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t {
            -(1 as libc::c_int) as uintmax_t
        } else {
            ((1 as libc::c_int as uintmax_t)
                << (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        };
        return LONGINT_OVERFLOW;
    }
    *x = scaled;
    return LONGINT_OK;
}
fn bkm_scale_by_power(
    x: &mut uintmax_t,
    base: libc::c_int,
    mut power: libc::c_int,
) -> strtol_error {
    let mut err: strtol_error = LONGINT_OK;
    while power > 0 {
        power -= 1;
        unsafe {
            err = (err as libc::c_uint | bkm_scale(x, base) as libc::c_uint) as strtol_error;
        }
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn xstrtoumax(
    mut s: *const libc::c_char,
    mut ptr: *mut *mut libc::c_char,
    mut strtol_base: libc::c_int,
    mut val: *mut uintmax_t,
    mut valid_suffixes: *const libc::c_char,
) -> strtol_error {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: uintmax_t = 0;
    let mut err: strtol_error = LONGINT_OK;
    if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
        __assert_fail(
            b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                as *const libc::c_char,
            b"./include/xstrtol.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"strtol_error xstrtoumax(const char *, char **, int, uintmax_t *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2256: {
        if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
            __assert_fail(
                b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                    as *const libc::c_char,
                b"./include/xstrtol.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"strtol_error xstrtoumax(const char *, char **, int, uintmax_t *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    p = if !ptr.is_null() { ptr } else { &mut t_ptr };
    *__errno_location() = 0 as libc::c_int;
    if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t {
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
    tmp = strtoumax(s, p, strtol_base);
    if *p == s as *mut libc::c_char {
        if !valid_suffixes.is_null() && **p as libc::c_int != 0
            && !(strchr(valid_suffixes, **p as libc::c_int)).is_null()
        {
            tmp = 1 as libc::c_int as uintmax_t;
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
                overflow = bkm_scale(&mut tmp, 512 as libc::c_int);
            }
            66 => {
                overflow = bkm_scale(&mut tmp, 1024 as libc::c_int);
            }
            99 => {
                overflow = LONGINT_OK;
            }
            69 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 6);
            }
            71 | 103 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 3);
            }
            107 | 75 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 1);
            }
            77 | 109 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 2);
            }
            80 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 5);
            }
            81 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 10);
            }
            82 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 9);
            }
            84 | 116 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 4);
            }
            119 => {
                overflow = bkm_scale(&mut tmp, 2 as libc::c_int);
            }
            89 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 8);
            }
            90 => {
                let mut overflow = bkm_scale_by_power(&mut tmp, base, 7);
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
