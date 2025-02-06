
















use std::vec;

use std::option::Option;

use std::convert::TryInto;

use std::slice;

use std::vec::Vec;

use std::string::String;

use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
    fn xalloc_die();
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub const DEFAULT_MXFAST_0: C2RustUnnamed_0 = 128;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
fn irealloc(s: i64) -> *mut libc::c_void {
    if s >= 0 && s <= usize::MAX as i64 {
        let size = s as usize;
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 0);
        let ptr = vec.as_mut_ptr();
        std::mem::forget(vec); // Prevent Vec from deallocating the memory
        ptr as *mut libc::c_void
    } else {
        unsafe {
    _gl_alloc_nomem()
}
    }
}

#[inline]
fn icalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    if n > usize::MAX / s {
        if s != 0 {
            return None; // _gl_alloc_nomem();
        }
        return Some(Vec::new()); // Allocating zero size
    }
    if s > usize::MAX / n {
        if n != 0 {
            return None; // _gl_alloc_nomem();
        }
        return Some(Vec::new()); // Allocating zero size
    }
    let total_size = n * s;
    Some(vec![0u8; total_size]) // Allocating and initializing memory
}

#[inline]
unsafe extern "C" fn ireallocarray(
    mut p: *mut libc::c_void,
    mut n: idx_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    if n as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
        && s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
    {
        let mut nx: size_t = n as size_t;
        let mut sx: size_t = s as size_t;
        if n == 0 as libc::c_int as libc::c_long || s == 0 as libc::c_int as libc::c_long
        {
            sx = 1 as libc::c_int as size_t;
            nx = sx;
        }
        p = reallocarray(p, nx, sx);
        return p;
    } else {
    return _gl_alloc_nomem();
};
}
#[cold]
#[inline]
fn _gl_alloc_nomem() -> *mut libc::c_void {
    // Set the error code to indicate out of memory
    unsafe { *__errno_location() = 12; }
    std::ptr::null_mut()
}

#[inline]
fn imalloc(s: usize) -> *mut libc::c_void {
    if s <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s, 1).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            std::ptr::null_mut()
        } else {
            ptr as *mut libc::c_void
        }
    } else {
        std::ptr::null_mut()
    }
}

fn check_nonnull(p: *mut libc::c_void) -> *mut libc::c_void {
    if p.is_null() {
        panic!("Pointer is null");
    }
    p
}

#[no_mangle]
pub fn xmalloc(s: usize) -> Vec<u8> {
    let mut vec = vec![0u8; s];
    check_nonnull(vec.as_mut_ptr() as *mut libc::c_void);
    vec
}

#[no_mangle]
pub unsafe extern "C" fn ximalloc(mut s: idx_t) -> *mut libc::c_void {
    let allocated_memory = imalloc(s.try_into().unwrap());
    check_nonnull(allocated_memory)
}
#[no_mangle]
pub fn xcharalloc(n: usize) -> Vec<libc::c_char> {
    let size = std::mem::size_of::<libc::c_char>();
    let ptr = if size == 1 {
        let n_bytes: usize = n.try_into().unwrap();
let mut allocated_memory = xmalloc(n_bytes);
let char_ptr = allocated_memory.as_mut_ptr() as *mut libc::c_char;
// Return or use char_ptr as needed
char_ptr
    } else {
        let n_usize: usize = n.try_into().unwrap();
let size_usize: usize = size.try_into().unwrap();
let allocated_memory_ptr = xnmalloc(n_usize, size_usize);
allocated_memory_ptr
    };
    let slice = unsafe { std::slice::from_raw_parts(ptr, n) };
    slice.to_vec()
}

#[no_mangle]
pub fn xrealloc(p: *mut libc::c_void, s: usize) -> *mut libc::c_void {
    if s == 0 {
        if !p.is_null() {
            unsafe { libc::free(p) };
        }
        return std::ptr::null_mut();
    }

    let r = unsafe { libc::realloc(p, s) };
    if r.is_null() && (p.is_null() || s != 0) {
        unsafe { xalloc_die() };
    }
    r
}

#[no_mangle]
pub fn xirealloc(p: Option<&mut [u8]>, s: usize) -> Option<Vec<u8>> {
    match p {
        Some(slice) => {
            let mut vec = Vec::from(slice);
            vec.resize(s, 0);
            Some(vec)
        },
        None => {
            let mut vec = Vec::with_capacity(s);
            vec.resize(s, 0);
            Some(vec)
        }
    }
}

#[no_mangle]
pub fn xreallocarray(
    p: *mut libc::c_void,
    n: u64,
    s: u64,
) -> *mut libc::c_void {
    if n == 0 || s == 0 {
        return std::ptr::null_mut();
    }
    
    let new_size = n.checked_mul(s).unwrap_or(0);
    let r = unsafe { reallocarray(p, n, s) };
    
    if r.is_null() && (p.is_null() || n != 0 && s != 0) {
        unsafe { xalloc_die() };
    }
    
    r
}

#[no_mangle]
pub fn xireallocarray(
    p: &[u8],
    n: usize,
    s: usize,
) -> Option<Vec<u8>> {
    let new_size = n.checked_mul(s)?;
    let mut new_vec = Vec::with_capacity(new_size);
    new_vec.extend_from_slice(&p[..p.len().min(new_size)]);
    Some(new_vec)
}

#[no_mangle]
pub fn xnmalloc(n: usize, s: usize) -> *mut libc::c_char {
    let total_size = n.checked_mul(s).expect("Overflow in size calculation");
    let mut allocated_memory = Vec::with_capacity(total_size);
    let ptr = allocated_memory.as_mut_ptr() as *mut libc::c_char;
    std::mem::forget(allocated_memory); // Prevent Vec from deallocating the memory
    ptr
}

#[no_mangle]
pub fn xinmalloc(n: idx_t, s: idx_t) -> Vec<u8> {
    let total_size: usize = (n as usize).checked_mul(s as usize).expect("Overflow in multiplication");
    Vec::with_capacity(total_size)
}

#[no_mangle]
pub fn x2realloc(
    p: Option<&mut [u8]>,
    ps: &mut usize,
) -> Option<Vec<u8>> {
    let new_size = ps.saturating_add(1);
    let mut new_vec = Vec::with_capacity(new_size);
    if let Some(data) = p {
        new_vec.extend_from_slice(data);
    }
    *ps = new_size;
    Some(new_vec)
}

#[no_mangle]
pub fn x2nrealloc(
    mut p: Option<Vec<u8>>,
    mut pn: &mut usize,
    s: usize,
) -> Option<Vec<u8>> {
    let mut n: usize = *pn;
    if p.is_none() {
        if n == 0 {
            n = (DEFAULT_MXFAST as usize).wrapping_div(s);
            n = n.wrapping_add((n == 0) as usize);
        }
    } else {
        let (new_n, overflowed) = n.overflowing_add((n >> 1).wrapping_add(1));
        n = new_n;
        if overflowed {
            panic!("Memory allocation failed due to overflow");
        }
    }
    let mut new_vec = p.unwrap_or_else(|| Vec::with_capacity(0));
    new_vec.resize(n * s, 0);
    *pn = n;
    Some(new_vec)
}

#[no_mangle]
pub unsafe extern "C" fn xpalloc(
    mut pa: *mut libc::c_void,
    mut pn: *mut idx_t,
    mut n_incr_min: idx_t,
    mut n_max: ptrdiff_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    let mut n0: idx_t = *pn;
    let mut n: idx_t = 0;
    let (fresh2, fresh3) = n0.overflowing_add(n0 >> 1 as libc::c_int);
    *(&mut n as *mut idx_t) = fresh2;
    if fresh3 {
        n = 9223372036854775807 as libc::c_long;
    }
    if 0 as libc::c_int as libc::c_long <= n_max && n_max < n {
        n = n_max;
    }
    let mut nbytes: idx_t = 0;
    let mut adjusted_nbytes: idx_t = (if if (0 as libc::c_int as idx_t)
        < -(1 as libc::c_int) as idx_t
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { n })
            - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { s })
            - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && (if s < 0 as libc::c_int as libc::c_long {
            if n < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        -(1 as libc::c_int) as idx_t
                    }) + s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    (n < -(1 as libc::c_int) as idx_t / s) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (s
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + -(1 as libc::c_int) as idx_t
                            >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) as idx_t / -s
                    }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                }
            } else {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as idx_t
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as idx_t)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as idx_t
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t) as libc::c_int
                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < n + 0 as libc::c_int as idx_t) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < n
                            && (-(1 as libc::c_int) as libc::c_long
                                - 0 as libc::c_int as idx_t)
                                < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as idx_t / s < n) as libc::c_int
                }
            }
        } else {
            if s == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else {
                if n < 0 as libc::c_int as libc::c_long {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as idx_t
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as idx_t)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t) as libc::c_int
                    }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < s + 0 as libc::c_int as idx_t) as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) as libc::c_long
                                - 0 as libc::c_int as idx_t)
                                < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        (0 as libc::c_int as idx_t / n < s) as libc::c_int
                    }
                } else {
                    (-(1 as libc::c_int) as idx_t / s < n) as libc::c_int
                }
            }
        }) != 0
    {
        let (fresh8, _fresh9) = n.overflowing_mul(s);
        *(&mut nbytes as *mut idx_t) = fresh8;
        1 as libc::c_int
    } else {
        let (fresh10, fresh11) = n.overflowing_mul(s);
        *(&mut nbytes as *mut idx_t) = fresh10;
        fresh11 as libc::c_int
    } != 0
    {
        if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        }
    } else {
        (if nbytes < DEFAULT_MXFAST_0 as libc::c_int as libc::c_long {
            DEFAULT_MXFAST_0 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_ulong
    }) as idx_t;
    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
        nbytes = adjusted_nbytes - adjusted_nbytes % s;
    }
    if pa.is_null() {
        *pn = 0 as libc::c_int as idx_t;
    }
    if n - n0 < n_incr_min
        && {
            let (fresh12, fresh13) = n0.overflowing_add(n_incr_min);
            *(&mut n as *mut idx_t) = fresh12;
            fresh13 as libc::c_int != 0
                || 0 as libc::c_int as libc::c_long <= n_max && n_max < n
                || (if (0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    && (if s < 0 as libc::c_int as libc::c_long {
                        if n < 0 as libc::c_int as libc::c_long {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    -(1 as libc::c_int) as idx_t
                                }) + s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (n < -(1 as libc::c_int) as idx_t / s) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (s
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + -(1 as libc::c_int) as idx_t
                                        >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    -(1 as libc::c_int) as idx_t / -s
                                }) <= -(1 as libc::c_int) as libc::c_long - n)
                                    as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as idx_t
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as idx_t
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as idx_t
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t) as libc::c_int
                            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < n + 0 as libc::c_int as idx_t) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < n
                                        && (-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as idx_t)
                                            < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                (0 as libc::c_int as idx_t / s < n) as libc::c_int
                            }
                        }
                    } else {
                        if s == 0 as libc::c_int as libc::c_long {
                            0 as libc::c_int
                        } else {
                            if n < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) + 0 as libc::c_int as idx_t
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as idx_t
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as idx_t
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t) as libc::c_int
                                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < s + 0 as libc::c_int as idx_t) as libc::c_int
                                    } else {
                                        ((-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as idx_t)
                                            < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    (0 as libc::c_int as idx_t / n < s) as libc::c_int
                                }
                            } else {
                                (-(1 as libc::c_int) as idx_t / s < n) as libc::c_int
                            }
                        }
                    }) != 0
                {
                    let (fresh18, _fresh19) = n.overflowing_mul(s);
                    *(&mut nbytes as *mut idx_t) = fresh18;
                    1 as libc::c_int
                } else {
                    let (fresh20, fresh21) = n.overflowing_mul(s);
                    *(&mut nbytes as *mut idx_t) = fresh20;
                    fresh21 as libc::c_int
                }) != 0
        }
    {
        xalloc_die();
    }
    pa = xrealloc(pa as *mut libc::c_void, nbytes as usize);
    *pn = n;
    return pa;
}
#[no_mangle]
pub fn xzalloc(s: usize) -> Vec<u8> {
    let size: usize = s.try_into().unwrap(); // Convert size_t to usize
    vec![0; size] // Allocate a vector of size `s` initialized with zeros
}

#[no_mangle]
pub fn xizalloc(s: usize) -> Vec<u8> {
    let size = s * std::mem::size_of::<u8>();
    let vec = vec![0u8; size];
    vec
}

#[no_mangle]
pub fn xcalloc(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in allocation size");
    let mut vec = vec![0u8; total_size];
    let ptr = vec.as_mut_ptr() as *mut libc::c_void;
    std::mem::forget(vec); // Prevent dropping the Vec and freeing the memory
    ptr
}

#[no_mangle]
pub fn xicalloc(n: idx_t, s: idx_t) -> Vec<u8> {
    let size: usize = (n * s).try_into().expect("Size conversion failed");
    let vec = vec![0u8; size];
    check_nonnull(vec.as_ptr() as *mut libc::c_void);
    vec
}

#[no_mangle]
pub fn xmemdup(p: &[u8]) -> Vec<u8> {
    let s = p.len();
    let mut new_vec = Vec::with_capacity(s);
    new_vec.copy_from_slice(p);
    new_vec
}

#[no_mangle]
pub fn ximemdup(p: &[u8]) -> Vec<u8> {
    let s = p.len();
    let mut new_vec = vec![0u8; s];
    new_vec.copy_from_slice(p);
    new_vec
}

#[no_mangle]
pub fn ximemdup0(p: &[u8]) -> Vec<u8> {
    let s = p.len();
    let mut result = vec![0; s + 1]; // Allocate space for the data plus a null terminator
    result[..s].copy_from_slice(p); // Copy the data
    result[s] = 0; // Null-terminate the string
    result
}

#[no_mangle]
pub fn xstrdup(string: &str) -> String {
    string.to_owned()
}

