use ::libc;
extern "C" {
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn rpl_fcntl(
    mut fd: libc::c_int,
    mut action: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut arg: ::core::ffi::VaListImpl;
    let mut result: libc::c_int = -(1 as libc::c_int);
    arg = args.clone();
    match action {
        0 => {
            let mut target: libc::c_int = arg.arg::<libc::c_int>();
            result = rpl_fcntl_DUPFD(fd, target);
        }
        1030 => {
            let mut target_0: libc::c_int = arg.arg::<libc::c_int>();
            result = rpl_fcntl_DUPFD_CLOEXEC(fd, target_0);
        }
        _ => {
            let mut current_block_7: u64;
            match action {
                1 => {
                    current_block_7 = 4046302689674688614;
                }
                3 => {
                    current_block_7 = 4046302689674688614;
                }
                1025 => {
                    current_block_7 = 6453289516101043606;
                }
                9 => {
                    current_block_7 = 13722137258147953758;
                }
                1032 => {
                    current_block_7 = 12766345184754617216;
                }
                1034 => {
                    current_block_7 = 6940526744265269810;
                }
                11 => {
                    current_block_7 = 7022052692272556539;
                }
                1033 => {
                    current_block_7 = 8871774344836507656;
                }
                0 => {
                    current_block_7 = 8871774344836507656;
                }
                1030 => {
                    current_block_7 = 5351856672558463576;
                }
                1026 => {
                    current_block_7 = 9332010126091029806;
                }
                2 => {
                    current_block_7 = 17966572639739034653;
                }
                4 => {
                    current_block_7 = 15280576717888187136;
                }
                8 => {
                    current_block_7 = 12013198723313890981;
                }
                1031 => {
                    current_block_7 = 12013198723313890981;
                }
                1024 | 10 => {
                    current_block_7 = 14034191772621753005;
                }
                _ => {
                    let mut p: *mut libc::c_void = arg.arg::<*mut libc::c_void>();
                    result = fcntl(fd, action, p);
                    current_block_7 = 7175849428784450219;
                }
            }
            match current_block_7 {
                4046302689674688614 => {
                    current_block_7 = 6453289516101043606;
                }
                8871774344836507656 => {
                    current_block_7 = 5351856672558463576;
                }
                12013198723313890981 => {
                    current_block_7 = 14034191772621753005;
                }
                _ => {}
            }
            match current_block_7 {
                6453289516101043606 => {
                    current_block_7 = 13722137258147953758;
                }
                5351856672558463576 => {
                    current_block_7 = 9332010126091029806;
                }
                _ => {}
            }
            match current_block_7 {
                13722137258147953758 => {
                    current_block_7 = 12766345184754617216;
                }
                9332010126091029806 => {
                    current_block_7 = 17966572639739034653;
                }
                _ => {}
            }
            match current_block_7 {
                12766345184754617216 => {
                    current_block_7 = 6940526744265269810;
                }
                17966572639739034653 => {
                    current_block_7 = 15280576717888187136;
                }
                _ => {}
            }
            match current_block_7 {
                6940526744265269810 => {
                    current_block_7 = 7022052692272556539;
                }
                15280576717888187136 => {
                    current_block_7 = 14034191772621753005;
                }
                _ => {}
            }
            match current_block_7 {
                7022052692272556539 => {
                    result = fcntl(fd, action);
                }
                14034191772621753005 => {
                    let mut x: libc::c_int = arg.arg::<libc::c_int>();
                    result = fcntl(fd, action, x);
                }
                _ => {}
            }
        }
    }
    return result;
}
unsafe extern "C" fn rpl_fcntl_DUPFD(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = fcntl(fd, 0 as libc::c_int, target);
    return result;
}
static mut have_dupfd_cloexec: libc::c_int = 0;
unsafe extern "C" fn rpl_fcntl_DUPFD_CLOEXEC(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if 0 as libc::c_int <= have_dupfd_cloexec {
        result = fcntl(fd, 1030 as libc::c_int, target);
        if 0 as libc::c_int <= result || *__errno_location() != 22 as libc::c_int {
            have_dupfd_cloexec = 1 as libc::c_int;
        } else {
            result = rpl_fcntl_DUPFD(fd, target);
            if result >= 0 as libc::c_int {
                have_dupfd_cloexec = -(1 as libc::c_int);
            }
        }
    } else {
        result = rpl_fcntl_DUPFD(fd, target);
    }
    if 0 as libc::c_int <= result && have_dupfd_cloexec == -(1 as libc::c_int) {
        let mut flags: libc::c_int = fcntl(result, 1 as libc::c_int);
        if flags < 0 as libc::c_int
            || fcntl(result, 2 as libc::c_int, flags | 1 as libc::c_int)
                == -(1 as libc::c_int)
        {
            let mut saved_errno: libc::c_int = *__errno_location();
            close(result);
            *__errno_location() = saved_errno;
            result = -(1 as libc::c_int);
        }
    }
    return result;
}
unsafe extern "C" fn run_static_initializers() {
    have_dupfd_cloexec = if 0 as libc::c_int != 0 {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
