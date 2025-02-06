














use std::string::String;

use std::ptr;

use std::option::Option;

use std::mem;

use std::convert::TryInto;

use std::alloc::{self, Layout};

use std::slice;

use std::vec::Vec;

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
fn irealloc(p: Option<&mut [u8]>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let new_size = if s == 0 { 1 } else { s };
        let mut vec = Vec::with_capacity(new_size);
        if let Some(data) = p {
            vec.extend_from_slice(data);
        }
        Some(vec)
    } else {
        None // Assuming _gl_alloc_nomem() returns None in this context
    }
}

#[inline]
fn icalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    if n > usize::MAX / s {
        if s != 0 {
            return None; // _gl_alloc_nomem();
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    if s > usize::MAX / n {
        if n != 0 {
            return None; // _gl_alloc_nomem();
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    let total_size = n * s;
    let mut vec = Vec::with_capacity(total_size);
    unsafe {
        // Initialize the memory to zero
        let ptr = vec.as_mut_ptr();
        std::ptr::write_bytes(ptr, 0, total_size);
    }
    Some(vec)
}

#[inline]
fn ireallocarray(
    p: &mut Vec<u8>,
    n: usize,
    s: usize,
) -> Option<&mut Vec<u8>> {
    if n <= usize::MAX && s <= usize::MAX {
        let mut nx = n;
        let mut sx = s;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        let new_size = nx.checked_mul(sx)?;
        p.resize(new_size, 0);
        Some(p)
    } else {
        None
    }
}

#[cold]
#[inline]
fn _gl_alloc_nomem() -> *mut libc::c_void {
    // Set the errno to ENOMEM (12)
    unsafe { *__errno_location() = 12; }
    std::ptr::null_mut()
}

#[inline]
fn imalloc(s: usize) -> *mut libc::c_void {
    if s <= usize::MAX {
        let layout = Layout::from_size_align(s, 1).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };
        if ptr.is_null() {
            std::ptr::null_mut()
        } else {
            ptr as *mut libc::c_void
        }
    } else {
        std::ptr::null_mut()
    }
}

fn check_nonnull<T>(p: *mut T) -> *mut T {
    if p.is_null() {
        unsafe { xalloc_die() };
    }
    p
}

#[no_mangle]
pub fn xmalloc(s: usize) -> Option<Vec<u8>> {
    let layout = Layout::from_size_align(s, std::mem::align_of::<u8>()).ok()?;
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { Vec::from_raw_parts(ptr, s, s) })
    }
}

#[no_mangle]
pub unsafe extern "C" fn ximalloc(mut s: idx_t) -> *mut libc::c_void {
    let allocated_memory = imalloc(s.try_into().unwrap());
return check_nonnull(allocated_memory);
}
#[no_mangle]
pub fn xcharalloc(n: usize) -> Vec<libc::c_char> {
    let size = std::mem::size_of::<libc::c_char>();
    let layout = Layout::from_size_align(n * size, size).expect("Invalid layout");
    
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    
    let vec = unsafe { Vec::from_raw_parts(ptr as *mut libc::c_char, n, n) };
    std::mem::forget(vec.clone()); // Prevent Vec from deallocating the memory
    vec
}

#[no_mangle]
pub fn xrealloc(p: *mut libc::c_void, s: usize) -> *mut libc::c_void {
    if s == 0 {
        if !p.is_null() {
            unsafe { alloc::dealloc(p as *mut u8, Layout::from_size_align(0, 1).unwrap()) };
        }
        return std::ptr::null_mut();
    }

    let layout = Layout::from_size_align(s, 1).unwrap();
    let r = unsafe { alloc::realloc(p as *mut u8, layout, s) };

    if r.is_null() && (p.is_null() || s != 0) {
        unsafe { xalloc_die() };
    }
    r as *mut libc::c_void
}

#[no_mangle]
pub fn xirealloc(p: Option<&mut [u8]>, s: usize) -> Option<Vec<u8>> {
    match p {
        Some(slice) => {
            let mut vec = Vec::with_capacity(s);
            vec.extend_from_slice(slice);
            Some(vec)
        },
        None => Some(vec![0; s]),
    }
}

#[no_mangle]
pub fn xreallocarray(n: usize, s: usize) -> Option<Vec<u8>> {
    let total_size = n.checked_mul(s)?;
    let mut vec = Vec::with_capacity(total_size);
    unsafe {
        let ptr = vec.as_mut_ptr();
        let result = std::alloc::realloc(ptr, Layout::array::<u8>(total_size).unwrap(), total_size);
        if result.is_null() && (n != 0 && s != 0) {
            xalloc_die();
        }
        vec.set_len(total_size);
    }
    Some(vec)
}

#[no_mangle]
pub fn xireallocarray(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in multiplication");
    let mut vec = Vec::with_capacity(total_size);
    // Assuming that the original function initializes the memory in some way,
    // we can return the vector's pointer.
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent Vec from deallocating the memory
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    let result = xreallocarray(n.try_into().unwrap(), s.try_into().unwrap());
match result {
    Some(mut vec) => return vec.as_mut_ptr() as *mut libc::c_void,
    None => return std::ptr::null_mut(),
}
}
#[no_mangle]
pub fn xinmalloc(n: idx_t, s: idx_t) -> Vec<u8> {
    let size = (n as usize).checked_mul(s as usize).expect("Overflow in allocation size");
    Vec::with_capacity(size)
}

#[no_mangle]
pub fn x2realloc(
    p: &[u8],
    ps: &mut usize,
) -> Vec<u8> {
    let new_size = *ps + 1; // Assuming we want to allocate one more byte
    let mut new_vec = Vec::with_capacity(new_size);
    new_vec.extend_from_slice(p);
    *ps = new_size;
    new_vec
}

#[no_mangle]
pub fn x2nrealloc(
    p: Option<&mut [u8]>,
    pn: &mut usize,
    s: usize,
) -> Option<Vec<u8>> {
    let mut n = *pn;
    if p.is_none() {
        if n == 0 {
            n = (DEFAULT_MXFAST as usize).wrapping_div(s);
            n = n.wrapping_add((n == 0) as usize);
        }
    } else {
        let (new_n, overflowed) = n.overflowing_add((n >> 1).wrapping_add(1));
        n = new_n;
        if overflowed {
            return None; // Handle allocation failure gracefully
        }
    }
    let new_vec = vec![0u8; n * s];
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
    pa = xrealloc(pa, nbytes as usize);
    *pn = n;
    return pa;
}
#[no_mangle]
pub fn xzalloc(s: usize) -> Vec<u8> {
    let layout = Layout::from_size_align(s, 1).expect("Invalid layout");
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        panic!("Memory allocation failed");
    }
    let mut vec = unsafe { Vec::from_raw_parts(ptr, 0, s) };
    vec.resize(s, 0);
    vec
}

#[no_mangle]
pub fn xizalloc(s: idx_t) -> Vec<u8> {
    let layout = Layout::from_size_align(s.try_into().unwrap(), 1).unwrap();
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        panic!("Memory allocation failed");
    }
    let vec = unsafe { Vec::from_raw_parts(ptr, s as usize, s as usize) };
    vec
}

#[no_mangle]
pub fn xcalloc(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in allocation size");
    let layout = Layout::from_size_align(total_size, 1).expect("Invalid layout");
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    ptr as *mut libc::c_void
}

#[no_mangle]
pub fn xicalloc(n: usize, s: usize) -> *mut libc::c_void {
    let layout = Layout::from_size_align(n * s, std::mem::align_of::<u8>()).expect("Invalid layout");
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    ptr as *mut libc::c_void
}

#[no_mangle]
pub fn xmemdup(p: &[u8]) -> Vec<u8> {
    let mut result = vec![0; p.len()];
    result.copy_from_slice(p);
    result
}

#[no_mangle]
pub fn ximemdup(p: &[u8]) -> Vec<u8> {
    let mut result = vec![0; p.len()];
    result.copy_from_slice(p);
    result
}

#[no_mangle]
pub fn ximemdup0(p: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(p.len() + 1);
    result.extend_from_slice(p);
    result.push(0); // Null terminator
    result
}

#[no_mangle]
pub fn xstrdup(string: &str) -> String {
    let len = string.len();
    let mut vec = Vec::with_capacity(len + 1);
    vec.extend_from_slice(string.as_bytes());
    vec.push(0); // Null-terminate the string
    String::from_utf8(vec).expect("Failed to convert to String")
}

