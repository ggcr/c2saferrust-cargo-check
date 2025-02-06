












use std::convert::TryFrom;

use std::ops::RangeInclusive;

use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isalnum(c: i32) -> bool {
    (c >= 48 && c <= 57) || (c >= 65 && c <= 90) || (c >= 97 && c <= 122)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isalpha(c: i32) -> bool {
    matches!(c, 97..=122 | 65..=90)
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
    matches!(c, 0..=31 | 127)
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
    matches!(c, 
        48..=57 | 97..=122 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 
        | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126
        | 65..=90
    )
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_islower(c: i32) -> bool {
    (97..=122).contains(&c)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isprint(c: i32) -> bool {
    match c {
        32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 
        | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 
        | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 
        | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 
        | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 
        | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 
        | 123 | 124 | 125 | 126 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_ispunct(c: i32) -> bool {
    matches!(c, 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58
              | 59 | 60 | 61 | 62 | 63 | 64 | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isspace(c: i32) -> bool {
    matches!(c, 32 | 9 | 10 | 11 | 12 | 13)
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
    if (65..=90).contains(&c) {
        return c - 'A' as i32 + 'a' as i32;
    }
    c
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_toupper(c: i32) -> i32 {
    if (97..=122).contains(&c) {
        return c - ('a' as i32) + ('A' as i32);
    }
    c
}

