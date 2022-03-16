use std::{ffi::CStr, borrow::Cow};

use gp_sys::{_CameraFile, gp_file_new, gp_file_free, gp_file_get_mime_type};

macro_rules! try_gp_unsafe {
    ($x:expr) => {
        match unsafe { $x } {
            0 => (),
            err => {
                panic!("I have failed you ;-( [{}]", err)
            }
        }
    }
}

pub struct CameraFile{
    inner: *mut ::gp_sys::_CameraFile,
}

impl CameraFile{
    pub fn new() -> CameraFile {
        let mut inner: *mut _CameraFile = std::ptr::null_mut();
        try_gp_unsafe!{
            gp_file_new(&mut inner)
        }
        CameraFile{inner}
    }

    pub fn mimetype(&self) -> Cow<str> {
        let mut mimetype: *const i8 = std::ptr::null_mut();
        try_gp_unsafe!{
            gp_file_get_mime_type(self.inner, &mut mimetype)
        }
        unsafe{
            CStr::from_ptr(mimetype).to_string_lossy()
        }
    }
}

impl Drop for CameraFile{
    fn drop(&mut self) {
        try_gp_unsafe!{
            gp_file_free(self.inner)
        }
    }
}
