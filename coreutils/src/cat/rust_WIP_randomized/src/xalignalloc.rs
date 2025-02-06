
use std::alloc::{alloc, Layout};

use ::libc;
extern "C" {
    fn aligned_alloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[inline]
fn alignalloc(alignment: usize, size: usize) -> *mut libc::c_void {
    let alignment = if alignment.is_power_of_two() && alignment > 0 {
        alignment
    } else {
        usize::MAX
    };
    
    let size = if size > 0 {
        size
    } else {
        usize::MAX
    };

    let ptr = unsafe { aligned_alloc(alignment as libc::c_ulong, size as libc::c_ulong) };
    if ptr.is_null() {
        std::ptr::null_mut()
    } else {
        ptr
    }
}

#[no_mangle]
pub fn xalignalloc(alignment: usize, size: usize) -> Box<[u8]> {
    let layout = Layout::from_size_align(size, alignment).expect("Invalid layout");
    let p = unsafe { alloc(layout) };
    if p.is_null() {
        unsafe { xalloc_die() };
    }
    let slice = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(p as *mut u8, size)) };
    slice
}

