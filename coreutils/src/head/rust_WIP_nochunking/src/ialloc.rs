


use std::mem;

use std::alloc;
use std::ptr;

use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn ireallocarray(
    p: Option<&mut [u8]>,
    n: idx_t,
    s: idx_t,
) -> Option<Vec<u8>> {
    if n as usize <= usize::MAX && s as usize <= usize::MAX {
        let mut nx: usize = n as usize;
        let mut sx: usize = s as usize;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        let new_size = nx.checked_mul(sx)?;
        let mut vec = match p {
            Some(existing) => {
                let mut new_vec = Vec::with_capacity(new_size);
                new_vec.copy_from_slice(existing);
                new_vec
            },
            None => Vec::with_capacity(new_size),
        };
        vec.resize(new_size, 0);
        Some(vec)
    } else {
        None // Assuming _gl_alloc_nomem() returns a null pointer, we return None here.
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn icalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    if n > usize::MAX {
        if s != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(vec![]);
    }
    if s > usize::MAX {
        if n != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(vec![]);
    }
    let total_size = n.checked_mul(s)?;
    Some(vec![0u8; total_size])
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<&mut [u8]>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let new_size = if s == 0 { 1 } else { s };
        let mut vec = p.map(|slice| Vec::from(slice)).unwrap_or_else(Vec::new);
        vec.resize(new_size, 0);
        Some(vec) // Return the Vec instead of a slice to avoid borrowing issues.
    } else {
        None // Assuming _gl_alloc_nomem() returns a null pointer, we return None here.
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn imalloc(s: usize) -> Option<Box<[u8]>> {
    if s <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s, 1).ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, s)) })
        }
    } else {
        None // Return None instead of calling _gl_alloc_nomem()
    }
}

#[no_mangle]
#[cold]
#[inline]
#[linkage = "external"]
pub fn _gl_alloc_nomem() -> *mut libc::c_void {
    unsafe {
        *__errno_location() = 12; // Set errno to ENOMEM
    }
    std::ptr::null_mut() // Return a null pointer
}

