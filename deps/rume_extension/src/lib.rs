use lib_impl::{
    rume_get_init_str_impl, rume_strings_split_impl, rume_use_foo_impl,
    STRING_SPLIT_BEHAVIOR_KEEP_TOKEN_, STRING_SPLIT_BEHAVIOR_SKIP_TOKEN_,
};
use std::os::raw::c_char;

mod lib_impl;
mod strings;
mod test_lib;
mod test_strings;

#[no_mangle]
pub static STRING_SPLIT_BEHAVIOR_KEEP_TOKEN: libc::c_int = STRING_SPLIT_BEHAVIOR_KEEP_TOKEN_;
#[no_mangle]
pub static STRING_SPLIT_BEHAVIOR_SKIP_TOKEN: libc::c_int = STRING_SPLIT_BEHAVIOR_SKIP_TOKEN_;

/// # Safety
/// This function is unsafe because it dereferences the `desc` pointer.
#[no_mangle]
pub unsafe extern "C" fn rume_extension_get_init_str(desc: *mut *mut c_char) -> i32 {
    rume_get_init_str_impl(desc)
}

#[repr(C)]
#[derive(Debug)]
pub enum Foo {
    A([f32; 2]),
}

/// # Safety
/// This function is unsafe because it dereferences the `test_param` pointer.
#[no_mangle]
pub unsafe extern "C" fn rume_extension_use_foo(test_param: Foo) -> *mut c_char {
    rume_use_foo_impl(test_param)
}

/// # Safety
/// This function is unsafe because it dereferences the `str_ptr` and `delim_str` pointers.
#[no_mangle]
pub unsafe extern "C" fn rume_extension_strings_split(
    str_ptr: *const c_char,
    delim_str: *const c_char,
    behavior_ptr: libc::c_int,
) -> *mut *mut c_char {
    rume_strings_split_impl(str_ptr, delim_str, behavior_ptr)
}
