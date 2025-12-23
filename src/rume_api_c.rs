use std::ffi::c_int;

#[repr(C)]
pub struct Rume {
    pub new_: extern "C" fn() -> *mut Rume,
    pub init: extern "C" fn(*mut Rume) -> c_int,
}
