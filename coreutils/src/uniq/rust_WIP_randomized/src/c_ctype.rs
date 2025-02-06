










use std::ops::RangeInclusive;

use std::convert::TryFrom;

use std::os::raw::c_int;

use std::char;

use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isalnum(c: i32) -> bool {
    match c {
        48..=57 | 65..=90 | 97..=122 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isalpha(c: i32) -> bool {
    (b'a'..=b'z').contains(&(c as u8)) || (b'A'..=b'Z').contains(&(c as u8))
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isascii(c: i32) -> bool {
    (0..=127).contains(&c)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isblank(c: i32) -> bool {
    c == ' ' as i32 || c == '\t' as i32
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_iscntrl(c: i32) -> bool {
    match c {
        7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16 | 17
        | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isdigit(c: i32) -> bool {
    matches!(c, 48..=57)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isgraph(c: i32) -> bool {
    match c {
        48..=57 | 97..=122 | 33..=47 | 58..=64 | 91..=96 | 123..=126 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_islower(c: i32) -> bool {
    (c >= 97 && c <= 122)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isprint(c: i32) -> bool {
    (32..=126).contains(&c) // This range includes all printable ASCII characters
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_ispunct(c: c_int) -> bool {
    match c as u8 {
        33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58
        | 59 | 60 | 61 | 62 | 63 | 64 | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125
        | 126 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isspace(c: i32) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isupper(c: i32) -> bool {
    matches!(c, 65..=90)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isxdigit(c: i32) -> bool {
    matches!(c, 48..=57 | 97..=102 | 65..=70)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_tolower(c: i32) -> i32 {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        return c - ('A' as i32 - 'a' as i32);
    }
    c
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_toupper(c: i32) -> i32 {
    if let Some(ch) = char::from_u32(c as u32) {
        if ch.is_ascii_lowercase() {
            return ch.to_ascii_uppercase() as i32;
        }
    }
    c
}

