use std::boxed::Box;

use std::alloc::Layout;

use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn aligned_alloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn alignalloc(alignment: usize, size: usize) -> Option<Box<[u8]>> {
    let alignment = if alignment.is_power_of_two() { alignment } else { usize::MAX };
    let size = if size > 0 { size } else { usize::MAX };

    let layout = Layout::from_size_align(size, alignment).ok()?;
    let ptr = unsafe { std::alloc::alloc(layout) };

    if ptr.is_null() {
        None
    } else {
        // Convert the raw pointer to a Box to ensure memory safety
        Some(unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, size)) })
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn alignfree(ptr: Box<dyn std::any::Any>) {
    // Assuming the original function is meant to free memory allocated with a specific allocator,
    // we can drop the Box to free the memory safely in Rust.
    drop(ptr);
}

