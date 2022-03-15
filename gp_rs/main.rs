mod gp;
use std::ffi::CStr;

use crate::gp::{gp_file_new, gp_file_get_mime_type, gp_file_free, CameraFile};

macro_rules! try_gp_unsafe {
    ($x:expr) => {
        match unsafe { $x } {
            0 => (),
            err => {
                eprintln!("I have failed you ;-( [{}]", err);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let mut file: *mut CameraFile = std::ptr::null_mut();
    try_gp_unsafe!{
        gp_file_new(&mut file)
    };

    let mut mimetype: *const i8 = std::ptr::null_mut();
    try_gp_unsafe!{
        gp_file_get_mime_type (file, &mut mimetype)
    };
    let m = unsafe{
        CStr::from_ptr(mimetype).to_string_lossy().to_string()
    };
    println!("mime: {:?}", m);

    try_gp_unsafe!{
        gp_file_free(file)
    };
}
