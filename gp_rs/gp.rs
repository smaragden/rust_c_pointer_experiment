#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CameraFile {
    pub mime_type: [::std::os::raw::c_char; 64usize],
}
pub type CameraFile = _CameraFile;

#[link(name = "gp")]
extern "C" {
    pub fn gp_file_new(file: *mut *mut CameraFile) -> ::std::os::raw::c_int;

    pub fn gp_file_get_mime_type(
        file: *mut CameraFile,
        mime_type: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn gp_file_free(file: *mut CameraFile) -> ::std::os::raw::c_int;
}
