use std::ptr;

use ::libc;
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
#[no_mangle]
pub fn umaxtostr(i: uintmax_t, buf: &mut [libc::c_char]) -> *mut libc::c_char {
    let mut p = buf.len() as isize - 1;
    buf[p as usize] = 0;

    let mut num = i;
    if num < 0 {
        loop {
            p -= 1;
            buf[p as usize] = ('0' as i32 as libc::c_ulong)
                .wrapping_sub((num % 10) as libc::c_ulong) as libc::c_char;
            num = (num as libc::c_ulong / 10) as uintmax_t;
            if num == 0 {
                break;
            }
        }
        p -= 1;
        buf[p as usize] = '-' as i32 as libc::c_char;
    } else {
        loop {
            p -= 1;
            buf[p as usize] = ('0' as i32 as libc::c_ulong)
                .wrapping_add((num % 10) as libc::c_ulong) as libc::c_char;
            num = (num as libc::c_ulong / 10) as uintmax_t;
            if num == 0 {
                break;
            }
        }
    }
    buf.get_mut(p as usize).map_or(std::ptr::null_mut(), |b| b as *mut libc::c_char)
}

