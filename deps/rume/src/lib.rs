use libc;
use std::ffi::CString;
use std::os::raw::c_char;

const RETURNED_STRING: &str = "Some string from Rust";

#[no_mangle]
pub unsafe extern "C" fn get_some_cstr(desc: *mut *mut c_char) -> isize {
    if desc.is_null() || !(*desc).is_null() {
        return libc::EINVAL as isize;
    }

    let val = CString::new(RETURNED_STRING).expect("Expecting we can allocate a CString");

    let m = libc::malloc(libc::strlen(val.as_ptr()) + 1) as *mut c_char;
    if m.is_null() {
        return libc::ENOMEM as isize;
    }

    *desc = m;
    libc::strcpy(*desc, val.as_ptr());

    0
}
