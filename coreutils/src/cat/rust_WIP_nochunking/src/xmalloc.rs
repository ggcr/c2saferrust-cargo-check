














use std::ptr::null_mut;

use std::slice;

use std::string::String;

use std::alloc::{alloc, dealloc, Layout};

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
fn irealloc(s: usize) -> *mut libc::c_void {
    if s <= usize::MAX {
        let new_size = if s == 0 { 1 } else { s };
        let p = vec![0u8; new_size].into_boxed_slice();
        Box::into_raw(p) as *mut libc::c_void
    } else {
        _gl_alloc_nomem()
    }
}

#[inline]
fn icalloc(n: i64, s: i64) -> *mut libc::c_void {
    let n_usize = n.try_into().unwrap_or(0);
    let s_usize = s.try_into().unwrap_or(0);

    if n_usize > usize::MAX / s_usize {
        if s_usize != 0 {
            return _gl_alloc_nomem();
        }
        return std::ptr::null_mut();
    }
    if s_usize > usize::MAX / n_usize {
        if n_usize != 0 {
            return _gl_alloc_nomem();
        }
        return std::ptr::null_mut();
    }
    let total_size = n_usize * s_usize;
    let vec = vec![0u8; total_size];
    let boxed_slice = vec.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut libc::c_void
}

#[inline]
fn ireallocarray(
    p: *mut libc::c_void,
    n: usize,
    s: usize,
) -> *mut libc::c_void {
    if n <= usize::MAX && s <= usize::MAX {
        let mut nx: usize = n;
        let mut sx: usize = s;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        let new_size = nx.checked_mul(sx).unwrap_or(0);
        let new_p = unsafe { libc::reallocarray(p, new_size, 1) };
        new_p
    } else {
        _gl_alloc_nomem()
    }
}

#[cold]
#[inline]
fn _gl_alloc_nomem() -> *mut libc::c_void {
    unsafe {
        *__errno_location() = 12; // Set errno to 12 (ENOMEM)
    }
    std::ptr::null_mut() // Return a null pointer
}

#[inline]
fn imalloc(s: usize) -> *mut u8 {
    if s <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s, 1).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            std::ptr::null_mut()
        } else {
            ptr
        }
    } else {
        std::ptr::null_mut()
    }
}

fn check_nonnull(p: *mut libc::c_void) -> *mut libc::c_void {
    if p.is_null() {
        unsafe { xalloc_die() };
    }
    p
}

#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut s: size_t) -> *mut libc::c_void {
    return check_nonnull(malloc(s));
}
#[no_mangle]
pub unsafe extern "C" fn ximalloc(mut s: idx_t) -> *mut libc::c_void {
    let allocated_memory = imalloc(s.try_into().expect("Conversion failed"));
if allocated_memory.is_null() {
    return std::ptr::null_mut();
}
return check_nonnull(allocated_memory as *mut libc::c_void);
}
#[no_mangle]
pub fn xcharalloc(n: usize) -> Vec<libc::c_char> {
    let layout = Layout::from_size_align(n, std::mem::align_of::<libc::c_char>())
        .expect("Failed to create layout");
    
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    
    let vec = unsafe { Vec::from_raw_parts(ptr as *mut libc::c_char, n, n) };
    vec
}

#[no_mangle]
pub fn xrealloc(p: *mut libc::c_void, s: usize) -> *mut libc::c_void {
    let mut vec = if p.is_null() {
        Vec::with_capacity(s)
    } else {
        let slice = unsafe { std::slice::from_raw_parts_mut(p as *mut u8, s) };
        Vec::from(slice)
    };
    vec.resize(s, 0);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent Vec from deallocating the memory
    ptr as *mut libc::c_void
}

#[no_mangle]
pub fn xirealloc(mut p: Vec<u8>, s: usize) -> Option<Vec<u8>> {
    let mut new_vec = Vec::with_capacity(s);
    if p.len() < s {
        new_vec.extend_from_slice(&p);
        new_vec.resize(s, 0);
        Some(new_vec)
    } else {
        Some(p)
    }
}

#[no_mangle]
pub unsafe extern "C" fn xreallocarray(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = reallocarray(p, n, s);
    if r.is_null() && (p.is_null() || n != 0 && s != 0) {
        xalloc_die();
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn xireallocarray(
    mut p: *mut libc::c_void,
    mut n: idx_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    let new_p = ireallocarray(p, n as usize, s as usize);
check_nonnull(new_p)
}
#[no_mangle]
pub unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    return xreallocarray(0 as *mut libc::c_void, n, s);
}
#[no_mangle]
pub fn xinmalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    let total_size = n.checked_mul(s)?;
    let vec = Vec::with_capacity(total_size);
    Some(vec)
}

#[no_mangle]
pub fn x2realloc(
    p: &mut Vec<u8>,
    ps: &mut usize,
) -> Vec<u8> {
    let new_size = *ps + 1; // Assuming we want to increase the size by 1
    p.resize(new_size, 0); // Resize the vector, initializing new elements to 0
    *ps = new_size; // Update the size
    p.clone() // Return a clone of the vector
}

#[no_mangle]
pub unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
    } else {
        let (fresh0, fresh1) = n
            .overflowing_add(
                (n >> 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        *(&mut n as *mut size_t) = fresh0;
        if fresh1 {
            xalloc_die();
        }
    }
    p = xreallocarray(p, n, s);
    *pn = n;
    return p;
}
#[no_mangle]
pub fn xpalloc(
    pa: Option<&mut Vec<u8>>,
    pn: &mut idx_t,
    n_incr_min: idx_t,
    n_max: ptrdiff_t,
    s: idx_t,
) -> Vec<u8> {
    let n0 = *pn;
    let mut n = n0.checked_add(n0 >> 1).unwrap_or(idx_t::MAX);
    
    if n_max >= 0 && n_max < n {
        n = n_max;
    }

    let mut nbytes = 0;
    let adjusted_nbytes = if (0 < -(1 as idx_t) && (n - 1) < 0 && (s - 1) < 0 && (s < 0 && n < 0)) || 
        (s < 0 && n < -(1 as idx_t) / s) || 
        (0 < s && n < 0) || 
        (s == 0 && n < 0) {
        return Vec::new(); // Handle allocation failure
    } else {
        n.checked_mul(s).unwrap_or(idx_t::MAX)
    };

    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
        nbytes = adjusted_nbytes - adjusted_nbytes % s;
    }

    if pa.is_none() {
        *pn = 0;
    }

    if n - n0 < n_incr_min {
        n = n0.checked_add(n_incr_min).unwrap_or(idx_t::MAX);
    }

    if nbytes > 0 {
        let new_vec = vec![0u8; nbytes as usize];
        *pn = n;
        return new_vec;
    }

    Vec::new() // Handle allocation failure
}

#[no_mangle]
pub fn xzalloc(s: usize) -> Vec<u8> {
    let layout = Layout::from_size_align(s, 1).expect("Invalid layout");
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        panic!("Memory allocation failed");
    }
    let vec = unsafe { Vec::from_raw_parts(ptr, s, s) };
    vec
}

#[no_mangle]
pub fn xizalloc(s: idx_t) -> Vec<u8> {
    let size = s as usize; // Convert idx_t to usize
    let vec = vec![0u8; size]; // Allocate a vector of the specified size, initialized to zero
    vec
}

#[no_mangle]
pub fn xcalloc(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in multiplication");
    let vec = vec![0u8; total_size];
    let boxed_slice = vec.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut libc::c_void // Convert Box<[u8]> to *mut c_void
}

#[no_mangle]
pub fn xicalloc(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in allocation size");
    let allocation = vec![0u8; total_size].into_boxed_slice();
    Box::into_raw(allocation) as *mut libc::c_void
}

#[no_mangle]
pub fn xmemdup(p: &[u8]) -> Vec<u8> {
    let mut result = vec![0; p.len()];
    result.copy_from_slice(p);
    result
}

#[no_mangle]
pub fn ximemdup(p: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(p.len());
    result.copy_from_slice(p);
    result
}

#[no_mangle]
pub fn ximemdup0(p: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(p.len() + 1);
    result.extend_from_slice(p);
    result.push(0); // Null-terminate the string
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

