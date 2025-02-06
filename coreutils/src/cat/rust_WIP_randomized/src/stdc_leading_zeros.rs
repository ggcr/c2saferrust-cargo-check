



use std::u32;

use std::convert::TryInto;

use std::mem;

use ::libc;
#[inline]
fn __gl_stdbit_clz(n: u32) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u32>() as i32)
    }
}

#[inline]
fn __gl_stdbit_clzl(n: u64) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u32) as i32
    }
}

#[inline]
fn __gl_stdbit_clzll(n: u64) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u32) as i32
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stdc_leading_zeros_ui(mut n: libc::c_uint) -> libc::c_uint {
    let result: u32 = __gl_stdbit_clz(n) as u32;
return result;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_uc(n: u8) -> u32 {
    let leading_zeros = unsafe { stdc_leading_zeros_ui(n as u32) };
    let size_diff = (std::mem::size_of::<u32>() - std::mem::size_of::<u8>()) as u32;
    leading_zeros.wrapping_sub(8u32.wrapping_mul(size_diff)) as u32
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_us(n: u16) -> u32 {
    // Call the unsafe function within an unsafe block
    let leading_zeros = unsafe { stdc_leading_zeros_ui(n as u32) };
    let size_difference = (std::mem::size_of::<u32>() - std::mem::size_of::<u16>()) * 8;
    leading_zeros.wrapping_sub(size_difference as u32)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_ul(n: u64) -> u32 {
    __gl_stdbit_clzl(n) as u32
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_ull(n: u64) -> u32 {
    return n.leading_zeros();
}

