
use std::option::Option;

use std::vec::Vec;

use std::vec;

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
        p.resize(nx * sx, 0);
        Some(p)
    } else {
        None
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn icalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    if n > usize::MAX / s {
        if s != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    if s > usize::MAX / n {
        if n != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    let total_size = n * s;
    let vec = vec![0u8; total_size]; // Allocate and initialize memory
    Some(vec)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<Vec<u8>>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let mut vec = p.unwrap_or_else(Vec::new);
        vec.resize(s, 0);
        Some(vec)
    } else {
        None
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn imalloc(s: idx_t) -> Option<Box<[u8]>> {
    if s as usize <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s as usize, 1).ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, s as usize)) })
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
    std::process::exit(12); // Exit with error code 12 to indicate no memory
}

