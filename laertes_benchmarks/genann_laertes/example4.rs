
extern "C" {
    
    #[no_mangle]
    fn fclose(_: * mut crate::example3::__sFILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn feof(_: * mut crate::example3::__sFILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgets(_: * mut std::os::raw::c_char, _: std::os::raw::c_int, _: * mut crate::example3::__sFILE)
     -> * mut std::os::raw::c_char;
    #[no_mangle]
    fn fopen(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut crate::example3::__sFILE;
    #[no_mangle]
    fn fseek(_: * mut crate::example3::__sFILE, _: std::os::raw::c_long, _: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(_: * const std::os::raw::c_char);
    #[no_mangle]
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn atof(_: * const std::os::raw::c_char) -> std::os::raw::c_double;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: * const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strtok(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char)
     -> * mut std::os::raw::c_char;
    
    
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub use crate::genann::__sFILEX;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = std::os::raw::c_longlong;
pub type fpos_t = std::os::raw::c_longlong;
// #[derive(Copy, Clone)]

pub type __sbuf = crate::example3::__sbuf;
// #[derive(Copy, Clone)]

pub type __sFILE = crate::example3::__sFILE;
pub type FILE = crate::example3::__sFILE;
pub type genann_actfun = Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
// #[derive(Copy, Clone)]

pub type genann = crate::example1::genann;
/* This example is to illustrate how to use GENANN.
 * It is NOT an example of good machine learning techniques.
 */
#[no_mangle]
pub static mut iris_data: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_iris_data() {
iris_data = b"example/iris.data\x00" as *const u8 as *const std::os::raw::c_char;}//;
#[no_mangle]
pub static mut input: * mut std::os::raw::c_double =
    (0 as * mut std::os::raw::c_double); unsafe fn laertes_init_input() {
input = 0 as *const std::os::raw::c_double as *mut std::os::raw::c_double;}//;
#[no_mangle]
pub static mut class: * mut std::os::raw::c_double =
    (0 as * mut std::os::raw::c_double); unsafe fn laertes_init_class() {
class = (0 as * mut f64);}//;
#[no_mangle]
pub static mut samples: std::os::raw::c_int = 0; unsafe fn laertes_init_samples() {
samples = 0;}//;
#[no_mangle]
pub static mut class_names: [* const std::os::raw::c_char; 3] =
    [(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),]; unsafe fn laertes_init_class_names() {
class_names = [b"Iris-setosa\x00" as *const u8 as *const std::os::raw::c_char,
     b"Iris-versicolor\x00" as *const u8 as *const std::os::raw::c_char,
     b"Iris-virginica\x00" as *const u8 as *const std::os::raw::c_char];}//;
#[no_mangle]
pub unsafe extern "C" fn load_data() {
    /* Load the iris data-set. */
    let mut in_0: * mut crate::example3::__sFILE =
        fopen(b"example/iris.data\x00" as *const u8 as *const std::os::raw::c_char,
              b"r\x00" as *const u8 as *const std::os::raw::c_char);
    if in_0.is_null() {
        printf(b"Could not open file: %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, iris_data);
        exit(1 as std::os::raw::c_int);
    }
    /* Loop through the data to get a count. */
    let mut line: [i8; 1024] = [0; 1024];
    while feof(in_0) == 0 &&
              !fgets(line.as_mut_ptr(), 1024 as std::os::raw::c_int, in_0).is_null() {
        samples += 1
    }
    fseek(in_0, 0 as std::os::raw::c_int as std::os::raw::c_long, 0 as std::os::raw::c_int);
    printf(b"Loading %d data points from %s\n\x00" as *const u8 as
               *const std::os::raw::c_char, samples, iris_data);
    /* Allocate memory for input and output data. */
    input =
        malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                    std::os::raw::c_ulong).wrapping_mul(samples as
                                                    std::os::raw::c_ulong).wrapping_mul(4
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_ulong))
            as *mut std::os::raw::c_double;
    class =
        malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                    std::os::raw::c_ulong).wrapping_mul(samples as
                                                    std::os::raw::c_ulong).wrapping_mul(3
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_ulong))
            as *mut std::os::raw::c_double;
    /* Read the file into our arrays. */
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < samples {
        let mut p: * mut f64 =
            input.offset((i * 4 as std::os::raw::c_int) as isize);
        let mut c: * mut f64 =
            class.offset((i * 3 as std::os::raw::c_int) as isize);
        let ref mut fresh0 = *c.offset(2 as std::os::raw::c_int as isize);
        *fresh0 = 0.0f64;
        let ref mut fresh1 = *c.offset(1 as std::os::raw::c_int as isize);
        *fresh1 = *fresh0;
        *c.offset(0 as std::os::raw::c_int as isize) = *fresh1;
        if fgets(line.as_mut_ptr(), 1024 as std::os::raw::c_int, in_0).is_null() {
            perror(b"fgets\x00" as *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        let mut split: * mut i8 =
            strtok(line.as_mut_ptr(),
                   b",\x00" as *const u8 as *const std::os::raw::c_char);
        j = 0 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            *p.offset(j as isize) = atof(split);
            split =
                strtok(0 as *mut std::os::raw::c_char,
                       b",\x00" as *const u8 as *const std::os::raw::c_char);
            j += 1
        }
        *split.offset(strlen(split).wrapping_sub(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
            = 0 as std::os::raw::c_int as std::os::raw::c_char;
        if strcmp(split, class_names[0 as std::os::raw::c_int as usize]) ==
               0 as std::os::raw::c_int {
            *c.offset(0 as std::os::raw::c_int as isize) = 1.0f64
        } else if strcmp(split, class_names[1 as std::os::raw::c_int as usize]) ==
                      0 as std::os::raw::c_int {
            *c.offset(1 as std::os::raw::c_int as isize) = 1.0f64
        } else if strcmp(split, class_names[2 as std::os::raw::c_int as usize]) ==
                      0 as std::os::raw::c_int {
            *c.offset(2 as std::os::raw::c_int as isize) = 1.0f64
        } else {
            printf(b"Unknown class %s.\n\x00" as *const u8 as
                       *const std::os::raw::c_char, split);
            exit(1 as std::os::raw::c_int);
        }
        i += 1
        /* printf("Data point %d is %f %f %f %f  ->   %f %f %f\n", i, p[0], p[1], p[2], p[3], c[0], c[1], c[2]); */
    }
    fclose(in_0);
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: * mut * mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 4.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Train an ANN on the IRIS dataset using backpropagation.\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    /* Load the data from file. */
    load_data();
    /* 4 inputs.
     * 1 hidden layer(s) of 4 neurons.
     * 3 outputs (1 per class)
     */
    let mut ann: * mut crate::example1::genann =
        genann_init(4 as std::os::raw::c_int, 1 as std::os::raw::c_int, 4 as std::os::raw::c_int,
                    3 as std::os::raw::c_int);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut loops: i32 = 5000 as std::os::raw::c_int;
    /* Train the network with backpropagation. */
    printf(b"Training for %d loops over data.\n\x00" as *const u8 as
               *const std::os::raw::c_char, loops);
    i = 0 as std::os::raw::c_int;
    while i < loops {
        j = 0 as std::os::raw::c_int;
        while j < samples {
            genann_train(ann, input.offset((j * 4 as std::os::raw::c_int) as isize),
                         class.offset((j * 3 as std::os::raw::c_int) as isize),
                         0.01f64);
            j += 1
        }
        i += 1
        /* printf("%1.2f ", xor_score(ann)); */
    }
    let mut correct: i32 = 0 as std::os::raw::c_int;
    j = 0 as std::os::raw::c_int;
    while j < samples {
        let mut guess: * const f64 =
            genann_run(ann, input.offset((j * 4 as std::os::raw::c_int) as isize));
        if *class.offset((j * 3 as std::os::raw::c_int + 0 as std::os::raw::c_int) as isize)
               == 1.0f64 {
            if *guess.offset(0 as std::os::raw::c_int as isize) >
                   *guess.offset(1 as std::os::raw::c_int as isize) &&
                   *guess.offset(0 as std::os::raw::c_int as isize) >
                       *guess.offset(2 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else if *class.offset((j * 3 as std::os::raw::c_int + 1 as std::os::raw::c_int) as
                                    isize) == 1.0f64 {
            if *guess.offset(1 as std::os::raw::c_int as isize) >
                   *guess.offset(0 as std::os::raw::c_int as isize) &&
                   *guess.offset(1 as std::os::raw::c_int as isize) >
                       *guess.offset(2 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else if *class.offset((j * 3 as std::os::raw::c_int + 2 as std::os::raw::c_int) as
                                    isize) == 1.0f64 {
            if *guess.offset(2 as std::os::raw::c_int as isize) >
                   *guess.offset(0 as std::os::raw::c_int as isize) &&
                   *guess.offset(2 as std::os::raw::c_int as isize) >
                       *guess.offset(1 as std::os::raw::c_int as isize) {
                correct += 1
            }
        } else {
            printf(b"Logic error.\n\x00" as *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        j += 1
    }
    printf(b"%d/%d correct (%0.1f%%).\n\x00" as *const u8 as
               *const std::os::raw::c_char, correct, samples,
           correct as std::os::raw::c_double / samples as std::os::raw::c_double * 100.0f64);
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
use crate::laertes_rt::*;
