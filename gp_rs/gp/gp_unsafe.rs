use std::ffi::CStr;

use gp_sys::{gp_file_new, gp_file_get_mime_type, gp_file_free, _CameraFile};



fn main() {
    println!("Running gp_unsafe");
    let mut file: *mut _CameraFile = std::ptr::null_mut();
    match unsafe {
        gp_file_new(&mut file)
    }{
        0 => (),
        err => panic!("gp_file_new: failed with errno {}", err)
    }

    let mut mimetype: *const i8 = std::ptr::null_mut();
    match unsafe {
        gp_file_get_mime_type (file, &mut mimetype)
    }{
        0 => (),
        err => panic!("gp_file_get_mime_type: failed with errno {}", err)
    }
    let m = unsafe{
        CStr::from_ptr(mimetype).to_string_lossy()
    };
    println!("mime: {:?}", m);

    match unsafe {
        gp_file_free(file)
    }{
        0 => (),
        err => panic!("gp_file_free: failed with errno {}", err)
    }
}
