
use std::process;

use std::alloc::{self, Layout};

use std::option::Option;

use std::vec::Vec;

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
    n: usize,
    s: usize,
) -> Option<Vec<u8>> {
    if n <= usize::MAX && s <= usize::MAX {
        let mut nx = n;
        let mut sx = s;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        let new_size = nx.checked_mul(sx)?;
        let mut vec = p.map(|slice| slice.to_vec()).unwrap_or_else(Vec::new);
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
        return Some(Vec::new()); // Return an empty Vec
    }
    if s > usize::MAX {
        if n != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    let total_size = n.checked_mul(s)?;
    Some(vec![0u8; total_size]) // Allocate a Vec filled with zeroes
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<Vec<u8>>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let new_size = if s == 0 { 1 } else { s };
        let mut new_vec = Vec::with_capacity(new_size);
        if let Some(old_vec) = p {
            new_vec.extend(old_vec);
        }
        Some(new_vec)
    } else {
        None // Assuming _gl_alloc_nomem() returns None in this context
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn imalloc(s: idx_t) -> Option<Vec<u8>> {
    if s as usize <= usize::MAX {
        let vec = Vec::with_capacity(s as usize);
        Some(vec)
    } else {
        None
    }
}

#[no_mangle]
#[cold]
#[inline]
#[linkage = "external"]
pub fn _gl_alloc_nomem() -> Option<Vec<u8>> {
    // Set the error to indicate out of memory
    std::process::abort(); // This simulates the behavior of returning a null pointer
}

