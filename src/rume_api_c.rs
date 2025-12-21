use std::ffi::c_int;

#[repr(C)]
pub struct Rume {
    pub new: fn() -> *mut Rume,
    pub init: fn() -> c_int,
}
