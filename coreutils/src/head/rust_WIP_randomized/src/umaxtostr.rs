use std::slice;
use std::str;

use ::libc;
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
#[no_mangle]
pub fn umaxtostr(i: uintmax_t, buf: &mut [libc::c_char]) -> &str {
    let mut p = buf.len() as isize - 1;
    buf[p as usize] = 0;

    if i < 0 {
        let mut num = i;
        loop {
            p -= 1;
            buf[p as usize] = ('0' as i32 as uintmax_t).wrapping_sub(num % 10) as libc::c_char;
            num /= 10;
            if num == 0 {
                break;
            }
        }
        p -= 1;
        buf[p as usize] = '-' as libc::c_char;
    } else {
        let mut num = i;
        loop {
            p -= 1;
            buf[p as usize] = ('0' as i32 as uintmax_t).wrapping_add(num % 10) as libc::c_char;
            num /= 10;
            if num == 0 {
                break;
            }
        }
    }
    let slice = &buf[p as usize..];
    let utf8_slice = unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
    std::str::from_utf8(utf8_slice).unwrap()
}

