#![feature(core_intrinsics)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature( extern_types,  register_tool)]
#![feature(const_mut_refs, const_fn_fn_ptr_basics)]
#![feature(c_variadic)]


extern "C" {
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn strncmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
               _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn XGetImage(_: *mut Display, _: Drawable, _: std::os::raw::c_int, _: std::os::raw::c_int,
                 _: std::os::raw::c_uint, _: std::os::raw::c_uint, _: std::os::raw::c_ulong,
                 _: std::os::raw::c_int) -> *mut XImage;
    #[no_mangle]
    fn XOpenDisplay(_: *const std::os::raw::c_char) -> *mut Display;
    #[no_mangle]
    fn XCreateFontCursor(_: *mut Display, _: std::os::raw::c_uint) -> Cursor;
    #[no_mangle]
    fn XRootWindow(_: *mut Display, _: std::os::raw::c_int) -> Window;
    #[no_mangle]
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    #[no_mangle]
    fn XAllowEvents(_: *mut Display, _: std::os::raw::c_int, _: Time) -> std::os::raw::c_int;
    #[no_mangle]
    fn XDefaultScreen(_: *mut Display) -> std::os::raw::c_int;
    #[no_mangle]
    fn XFreeCursor(_: *mut Display, _: Cursor) -> std::os::raw::c_int;
    #[no_mangle]
    fn XGrabPointer(_: *mut Display, _: Window, _: std::os::raw::c_int,
                    _: std::os::raw::c_uint, _: std::os::raw::c_int, _: std::os::raw::c_int,
                    _: Window, _: Cursor, _: Time) -> std::os::raw::c_int;
    #[no_mangle]
    fn XQueryColor(_: *mut Display, _: Colormap, _: *mut XColor)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn XTranslateCoordinates(_: *mut Display, _: Window, _: Window,
                             _: std::os::raw::c_int, _: std::os::raw::c_int,
                             _: *mut std::os::raw::c_int, _: *mut std::os::raw::c_int,
                             _: *mut Window) -> std::os::raw::c_int;
    #[no_mangle]
    fn XUngrabPointer(_: *mut Display, _: Time) -> std::os::raw::c_int;
    #[no_mangle]
    fn XWindowEvent(_: *mut Display, _: Window, _: std::os::raw::c_long,
                    _: *mut XEvent) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::os::raw::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut std::os::raw::c_void,
    pub __pad2: *mut std::os::raw::c_void,
    pub __pad3: *mut std::os::raw::c_void,
    pub __pad4: *mut std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: std::os::raw::c_int,
}
pub type FILE = _IO_FILE;
pub type XID = std::os::raw::c_ulong;
pub type Atom = std::os::raw::c_ulong;
pub type VisualID = std::os::raw::c_ulong;
pub type Time = std::os::raw::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type XPointer = *mut std::os::raw::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: std::os::raw::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(_: *mut _XExtData)
                                 -> std::os::raw::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: std::os::raw::c_int,
    pub red_mask: std::os::raw::c_ulong,
    pub green_mask: std::os::raw::c_ulong,
    pub blue_mask: std::os::raw::c_ulong,
    pub bits_per_rgb: std::os::raw::c_int,
    pub map_entries: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: std::os::raw::c_int,
    pub nvisuals: std::os::raw::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub mwidth: std::os::raw::c_int,
    pub mheight: std::os::raw::c_int,
    pub ndepths: std::os::raw::c_int,
    pub depths: *mut Depth,
    pub root_depth: std::os::raw::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: std::os::raw::c_ulong,
    pub black_pixel: std::os::raw::c_ulong,
    pub max_maps: std::os::raw::c_int,
    pub min_maps: std::os::raw::c_int,
    pub backing_store: std::os::raw::c_int,
    pub save_unders: std::os::raw::c_int,
    pub root_input_mask: std::os::raw::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: std::os::raw::c_int,
    pub bits_per_pixel: std::os::raw::c_int,
    pub scanline_pad: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub xoffset: std::os::raw::c_int,
    pub format: std::os::raw::c_int,
    pub data: *mut std::os::raw::c_char,
    pub byte_order: std::os::raw::c_int,
    pub bitmap_unit: std::os::raw::c_int,
    pub bitmap_bit_order: std::os::raw::c_int,
    pub bitmap_pad: std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
    pub bytes_per_line: std::os::raw::c_int,
    pub bits_per_pixel: std::os::raw::c_int,
    pub red_mask: std::os::raw::c_ulong,
    pub green_mask: std::os::raw::c_ulong,
    pub blue_mask: std::os::raw::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option<unsafe extern "C" fn(_: *mut _XDisplay,
                                                  _: *mut Visual,
                                                  _: std::os::raw::c_uint,
                                                  _: std::os::raw::c_int,
                                                  _: std::os::raw::c_int,
                                                  _: *mut std::os::raw::c_char,
                                                  _: std::os::raw::c_uint,
                                                  _: std::os::raw::c_uint,
                                                  _: std::os::raw::c_int,
                                                  _: std::os::raw::c_int)
                                 -> *mut _XImage>,
    pub destroy_image: Option<unsafe extern "C" fn(_: *mut _XImage)
                                  -> std::os::raw::c_int>,
    pub get_pixel: Option<unsafe extern "C" fn(_: *mut _XImage,
                                               _: std::os::raw::c_int, _: std::os::raw::c_int)
                              -> std::os::raw::c_ulong>,
    pub put_pixel: Option<unsafe extern "C" fn(_: *mut _XImage,
                                               _: std::os::raw::c_int, _: std::os::raw::c_int,
                                               _: std::os::raw::c_ulong)
                              -> std::os::raw::c_int>,
    pub sub_image: Option<unsafe extern "C" fn(_: *mut _XImage,
                                               _: std::os::raw::c_int, _: std::os::raw::c_int,
                                               _: std::os::raw::c_uint,
                                               _: std::os::raw::c_uint)
                              -> *mut _XImage>,
    pub add_pixel: Option<unsafe extern "C" fn(_: *mut _XImage,
                                               _: std::os::raw::c_long)
                              -> std::os::raw::c_int>,
}
pub type XImage = _XImage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: std::os::raw::c_ulong,
    pub red: std::os::raw::c_ushort,
    pub green: std::os::raw::c_ushort,
    pub blue: std::os::raw::c_ushort,
    pub flags: std::os::raw::c_char,
    pub pad: std::os::raw::c_char,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: std::os::raw::c_int,
    pub private2: std::os::raw::c_int,
    pub proto_major_version: std::os::raw::c_int,
    pub proto_minor_version: std::os::raw::c_int,
    pub vendor: *mut std::os::raw::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: std::os::raw::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                                   -> XID>,
    pub byte_order: std::os::raw::c_int,
    pub bitmap_unit: std::os::raw::c_int,
    pub bitmap_pad: std::os::raw::c_int,
    pub bitmap_bit_order: std::os::raw::c_int,
    pub nformats: std::os::raw::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: std::os::raw::c_int,
    pub release: std::os::raw::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: std::os::raw::c_int,
    pub last_request_read: std::os::raw::c_ulong,
    pub request: std::os::raw::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: std::os::raw::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                              -> std::os::raw::c_int>,
    pub display_name: *mut std::os::raw::c_char,
    pub default_screen: std::os::raw::c_int,
    pub nscreens: std::os::raw::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: std::os::raw::c_ulong,
    pub private16: std::os::raw::c_ulong,
    pub min_keycode: std::os::raw::c_int,
    pub max_keycode: std::os::raw::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: std::os::raw::c_int,
    pub xdefaults: *mut std::os::raw::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub x_root: std::os::raw::c_int,
    pub y_root: std::os::raw::c_int,
    pub state: std::os::raw::c_uint,
    pub keycode: std::os::raw::c_uint,
    pub same_screen: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub x_root: std::os::raw::c_int,
    pub y_root: std::os::raw::c_int,
    pub state: std::os::raw::c_uint,
    pub button: std::os::raw::c_uint,
    pub same_screen: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub x_root: std::os::raw::c_int,
    pub y_root: std::os::raw::c_int,
    pub state: std::os::raw::c_uint,
    pub is_hint: std::os::raw::c_char,
    pub same_screen: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub x_root: std::os::raw::c_int,
    pub y_root: std::os::raw::c_int,
    pub mode: std::os::raw::c_int,
    pub detail: std::os::raw::c_int,
    pub same_screen: std::os::raw::c_int,
    pub focus: std::os::raw::c_int,
    pub state: std::os::raw::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: std::os::raw::c_int,
    pub detail: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [std::os::raw::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
    pub major_code: std::os::raw::c_int,
    pub minor_code: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: std::os::raw::c_int,
    pub minor_code: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub border_width: std::os::raw::c_int,
    pub override_redirect: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub override_redirect: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub border_width: std::os::raw::c_int,
    pub above: Window,
    pub override_redirect: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub border_width: std::os::raw::c_int,
    pub above: Window,
    pub detail: std::os::raw::c_int,
    pub value_mask: std::os::raw::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: std::os::raw::c_int,
    pub state: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: std::os::raw::c_int,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [std::os::raw::c_char; 20],
    pub s: [std::os::raw::c_short; 10],
    pub l: [std::os::raw::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: std::os::raw::c_int,
    pub first_keycode: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: std::os::raw::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: std::os::raw::c_ulong,
    pub error_code: std::os::raw::c_uchar,
    pub request_code: std::os::raw::c_uchar,
    pub minor_code: std::os::raw::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub extension: std::os::raw::c_int,
    pub evtype: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: std::os::raw::c_int,
    pub display: *mut Display,
    pub extension: std::os::raw::c_int,
    pub evtype: std::os::raw::c_int,
    pub cookie: std::os::raw::c_uint,
    pub data: *mut std::os::raw::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: std::os::raw::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [std::os::raw::c_long; 24],
}
pub type XEvent = _XEvent;
pub type XErrorHandler
    =
    Option<unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent)
               -> std::os::raw::c_int>;
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut display: *mut Display = 0 as *mut Display;
    let mut status: std::os::raw::c_int = 0;
    let mut color: XColor =
        XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
    let mut cmap: Colormap = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut g: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strncmp(*argv.offset(i as isize),
                   b"-h\x00" as *const u8 as *const std::os::raw::c_char,
                   2 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"grabc 1.1 by Muhammad A Muquit\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        i += 1
    }
    display = XOpenDisplay(0 as *mut std::os::raw::c_void as *mut std::os::raw::c_char);
    cmap =
        (*(*(display as
                 _XPrivDisplay)).screens.offset((*(display as
                                                       _XPrivDisplay)).default_screen
                                                    as isize)).cmap;
    XSetErrorHandler(Some(MXError as
                              unsafe extern "C" fn(_: *mut Display,
                                                   _: *mut XErrorEvent)
                                  -> std::os::raw::c_int));
    if display.is_null() {
        fprintf(stderr,
                b"Failed to open local DISPLAY!\'n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    status = getWindowColor(display, &mut color);
    if status == 1 as std::os::raw::c_int {
        XQueryColor(display, cmap, &mut color);
        r = color.red as std::os::raw::c_int >> 8 as std::os::raw::c_int;
        g = color.green as std::os::raw::c_int >> 8 as std::os::raw::c_int;
        b = color.blue as std::os::raw::c_int >> 8 as std::os::raw::c_int;
        fprintf(stdout,
                b"#%02x%02x%02x\n\x00" as *const u8 as *const std::os::raw::c_char, r,
                g, b);
        fflush(stdout);
        /*
        ** write the values in decimal on stderr
        */
        fprintf(stderr, b"%d,%d,%d\n\x00" as *const u8 as *const std::os::raw::c_char,
                r, g, b);
    } else {
        fprintf(stderr,
                b"Failed to grab color!\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    return 0 as std::os::raw::c_int;
}
// Initialized in run_static_initializers
static mut cross_cursor: Cursor = 0;
/*  A program to pick a color by clicking the mouse.
 *
 *  RCS:
 *      $Revision$
 *      $Date$
 *
 *  Description:
 *
 *  When this program is run, the mouse pointer is grabbed and changed to
 *  a cross hair and when the mouse is clicked, the color of the clicked
 *  pixel is written to stdout in hex prefixed with #
 *
 *  This program can be useful when you see a color and want to use the
 *  color in xterm or your window manager's border but no clue what the 
 *  name of the color is. It's silly to use a image processing software
 *  to find it out.
 *
 * Example: 
 *   cpick
 *          #ffffff
 *   xterm -bg `cpick` -fg `cpick` (silly but esoteric!) 
 *
 *  Development History:
 *      who                  when               why
 *      ma_muquit@fccc.edu   march-16-19997     first cut
 */
/* private function prototypes */
/*
** function to select a window
** output parameters: x,y (coordinate of the point of click)
** reutrns Window
** exits if mouse can not be grabbed
*/
unsafe extern "C" fn selectWindow(mut display: *mut Display,
                                  mut x: *mut std::os::raw::c_int,
                                  mut y: *mut std::os::raw::c_int) -> Window {
    let mut target_cursor: Cursor = 0;
    let mut status: std::os::raw::c_int = 0;
    let mut target_window: Window = 0;
    let mut root_window: Window = 0;
    let mut event: XEvent = _XEvent{type_0: 0,};
    target_window = 0 as *mut std::os::raw::c_void as Window;
    if cross_cursor == 0 as *mut std::os::raw::c_void as Cursor {
        cross_cursor =
            XCreateFontCursor(display, 130 as std::os::raw::c_int as std::os::raw::c_uint);
        if cross_cursor == 0 as *mut std::os::raw::c_void as Cursor {
            fprintf(stderr,
                    b"Failed to create Cross Cursor!\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            return 0 as *mut std::os::raw::c_void as Window
        }
    }
    target_cursor = cross_cursor;
    root_window = XRootWindow(display, XDefaultScreen(display));
    status =
        XGrabPointer(display, root_window, 0 as std::os::raw::c_int,
                     ((1 as std::os::raw::c_long) << 2 as std::os::raw::c_int) as
                         std::os::raw::c_uint, 0 as std::os::raw::c_int, 1 as std::os::raw::c_int,
                     root_window, target_cursor, 0 as std::os::raw::c_long as Time);
    if status == 0 as std::os::raw::c_int {
        XAllowEvents(display, 1 as std::os::raw::c_int, 0 as std::os::raw::c_long as Time);
        XWindowEvent(display, root_window,
                     (1 as std::os::raw::c_long) << 2 as std::os::raw::c_int, &mut event);
        if event.type_0 == 4 as std::os::raw::c_int {
            target_window =
                findSubWindow(display, root_window, event.xbutton.subwindow,
                              &mut event.xbutton.x, &mut event.xbutton.y);
            if target_window == 0 as *mut std::os::raw::c_void as Window {
                fprintf(stderr,
                        b"Failed to get target window, getting root window!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
                target_window = root_window
            }
            XUngrabPointer(display, 0 as std::os::raw::c_long as Time);
        }
    } else {
        fprintf(stderr,
                b"Failed to grab mouse!\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    /* free things we do not need, always a good practice */
    XFreeCursor(display, cross_cursor);
    *x = event.xbutton.x;
    *y = event.xbutton.y;
    return target_window;
}
/* find a window */
unsafe extern "C" fn findSubWindow(mut display: *mut Display,
                                   mut top_window: Window,
                                   mut window_to_check: Window,
                                   mut x: *mut std::os::raw::c_int,
                                   mut y: *mut std::os::raw::c_int) -> Window {
    let mut newx: std::os::raw::c_int = 0;
    let mut newy: std::os::raw::c_int = 0;
    let mut window: Window = 0;
    if top_window == 0 as *mut std::os::raw::c_void as Window {
        return 0 as *mut std::os::raw::c_void as Window
    }
    if window_to_check == 0 as *mut std::os::raw::c_void as Window {
        return 0 as *mut std::os::raw::c_void as Window
    }
    /* initialize automatics */
    window = window_to_check;
    while XTranslateCoordinates(display, top_window, window_to_check, *x, *y,
                                &mut newx, &mut newy, &mut window) !=
              0 as std::os::raw::c_int && window != 0 as *mut std::os::raw::c_void as Window {
        if window != 0 as *mut std::os::raw::c_void as Window {
            top_window = window_to_check;
            window_to_check = window;
            *x = newx;
            *y = newy
        }
    }
    if window == 0 as *mut std::os::raw::c_void as Window { window = window_to_check }
    *x = newx;
    *y = newy;
    return window;
}
/*
 * get the color of the pixel of the point of mouse click
 * output paramter: XColor *color
 *
 * returns True if succeeds
 *          False if fails
 */
unsafe extern "C" fn getWindowColor(mut display: *mut Display,
                                    mut color: *mut XColor) -> std::os::raw::c_int {
    let mut root_window: Window = 0;
    let mut target_window: Window = 0;
    let mut ximage: *mut XImage = 0 as *mut XImage;
    let mut x: std::os::raw::c_int = 0;
    let mut y: std::os::raw::c_int = 0;
    let mut status: std::os::raw::c_int = 0;
    root_window = XRootWindow(display, XDefaultScreen(display));
    target_window = selectWindow(display, &mut x, &mut y);
    if target_window == 0 as *mut std::os::raw::c_void as Window {
        return 0 as std::os::raw::c_int
    }
    ximage =
        XGetImage(display, target_window, x, y,
                  1 as std::os::raw::c_int as std::os::raw::c_uint,
                  1 as std::os::raw::c_int as std::os::raw::c_uint,
                  !(0 as std::os::raw::c_long) as std::os::raw::c_ulong, 2 as std::os::raw::c_int);
    if ximage.is_null() { return 0 as std::os::raw::c_int }
    (*color).pixel =
        Some((*ximage).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")(ximage,
                                                                                                            0
                                                                                                                as
                                                                                                                std::os::raw::c_int,
                                                                                                            0
                                                                                                                as
                                                                                                                std::os::raw::c_int);
    Some((*ximage).f.destroy_image.expect("non-null function pointer")).expect("non-null function pointer")(ximage);
    return 1 as std::os::raw::c_int;
}
/* forgiving X error handler */
unsafe extern "C" fn MXError(mut display: *mut Display,
                             mut error: *mut XErrorEvent) -> std::os::raw::c_int {
    let mut xerrcode: std::os::raw::c_int = 0;
    xerrcode = (*error).error_code as std::os::raw::c_int;
    if xerrcode == 11 as std::os::raw::c_int ||
           xerrcode == 10 as std::os::raw::c_int &&
               (*error).request_code as std::os::raw::c_int == 88 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    } else {
        match (*error).request_code as std::os::raw::c_int {
            14 => {
                if (*error).error_code as std::os::raw::c_int == 9 as std::os::raw::c_int {
                    return 0 as std::os::raw::c_int
                }
            }
            3 | 15 => {
                if (*error).error_code as std::os::raw::c_int == 3 as std::os::raw::c_int {
                    return 0 as std::os::raw::c_int
                }
            }
            91 => {
                if (*error).error_code as std::os::raw::c_int == 2 as std::os::raw::c_int {
                    return 0 as std::os::raw::c_int
                }
            }
            _ => { }
        }
    }
    return 1 as std::os::raw::c_int;
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
unsafe extern "C" fn run_static_initializers() {
    cross_cursor = 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void as Cursor
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
