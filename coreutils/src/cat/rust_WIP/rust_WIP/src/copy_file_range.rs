use ::libc;
extern "C" {
    fn copy_file_range(
        __infd: libc::c_int,
        __pinoff: *mut __off64_t,
        __outfd: libc::c_int,
        __poutoff: *mut __off64_t,
        __length: size_t,
        __flags: libc::c_uint,
    ) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[no_mangle]
pub unsafe extern "C" fn rpl_copy_file_range(
    mut infd: libc::c_int,
    mut pinoff: *mut off_t,
    mut outfd: libc::c_int,
    mut poutoff: *mut off_t,
    mut length: size_t,
    mut flags: libc::c_uint,
) -> ssize_t {
    static mut ok: libc::c_schar = 0;
    if ok == 0 {
        let mut name: utsname = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };
        uname(&mut name);
        let mut p: *mut libc::c_char = (name.release).as_mut_ptr();
        ok = (if *p.offset(1 as libc::c_int as isize) as libc::c_int != '.' as i32
            || ('5' as i32) < *p.offset(0 as libc::c_int as isize) as libc::c_int
            || *p.offset(0 as libc::c_int as isize) as libc::c_int == '5' as i32
                && (*p.offset(3 as libc::c_int as isize) as libc::c_int != '.' as i32
                    || ('2' as i32)
                        < *p.offset(2 as libc::c_int as isize) as libc::c_int)
        {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar;
    }
    if (0 as libc::c_int) < ok as libc::c_int {
        return copy_file_range(infd, pinoff, outfd, poutoff, length, flags);
    }
    *__errno_location() = 38 as libc::c_int;
    return -(1 as libc::c_int) as ssize_t;
}
