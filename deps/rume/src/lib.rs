use std::ffi::CString;
use std::os::raw::c_char;
use strings::{split, SplitBehavior};

mod strings;
mod test_lib;
mod test_strings;

const RETURNED_STRING: &str = "Some string from Rust";

// https://github.com/mozilla/cbindgen/tree/master/tests/rust

fn rume_get_init_str_impl(desc: *mut *mut c_char) -> i32 {
    if desc.is_null() || unsafe { !(*desc).is_null() } {
        return libc::EINVAL;
    }

    let val = CString::new(RETURNED_STRING).expect("Expecting we can allocate a CString");

    let m = unsafe { libc::malloc(val.as_bytes().len() + 1) as *mut c_char };
    if m.is_null() {
        return libc::ENOMEM;
    }

    unsafe {
        *desc = m;
        libc::strcpy(*desc, val.as_ptr());
    }

    0
}

/// # Safety
/// This function is unsafe because it dereferences the `desc` pointer.
#[no_mangle]
pub unsafe extern "C" fn rume_get_init_str(desc: *mut *mut c_char) -> i32 {
    rume_get_init_str_impl(desc)
}

impl From<libc::c_int> for SplitBehavior {
    fn from(val: libc::c_int) -> Self {
        match val {
            0 => SplitBehavior::KeepToken,
            1 => SplitBehavior::SkipToken,
            _ => panic!("Invalid value for SplitBehavior"),
        }
    }
}

#[no_mangle]
pub static STRING_SPLIT_BEHAVIOR_KEEP_TOKEN: libc::c_int = SplitBehavior::KeepToken as libc::c_int;
#[no_mangle]
pub static STRING_SPLIT_BEHAVIOR_SKIP_TOKEN: libc::c_int = SplitBehavior::SkipToken as libc::c_int;

#[repr(C)]
#[derive(Debug)]
pub enum Foo {
    A([f32; 2]),
}

fn rume_strings_split_impl(
    str_ptr: *const c_char,
    delim_str: *const c_char,
    behavior_ptr: *const libc::c_int,
) -> *mut *mut c_char {
    let behavior = if behavior_ptr.is_null() {
        None
    } else {
        Some(unsafe { *behavior_ptr }.into())
    };

    let str = unsafe { std::ffi::CStr::from_ptr(str_ptr) }
        .to_str()
        .expect("Expecting valid UTF-8 string");
    let delim = unsafe { std::ffi::CStr::from_ptr(delim_str) }
        .to_str()
        .expect("Expecting valid UTF-8 string");
    let result = split(str, delim, behavior);

    let return_ptr = unsafe {
        libc::malloc(result.len() * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char
    };
    if return_ptr.is_null() {
        return std::ptr::null_mut();
    }

    for (i, s) in result.iter().enumerate() {
        let c_str = CString::new(s.as_str()).expect("Expecting we can allocate a CString");
        let m = unsafe { libc::malloc(c_str.as_bytes().len() + 1) as *mut c_char };
        if m.is_null() {
            return std::ptr::null_mut();
        }

        unsafe {
            *return_ptr.add(i) = m;
            libc::strcpy(*return_ptr.add(i), c_str.as_ptr());
        }
    }

    return_ptr
}

/// # Safety
/// This function is unsafe because it dereferences the `test_param` pointer.
#[no_mangle]
pub unsafe extern "C" fn rume_use_foo(test_param: Foo) -> *mut c_char {
    let str = format!("{:?}", test_param);
    let c_str = CString::new(str).expect("Expecting we can allocate a CString");
    let m = unsafe { libc::malloc(c_str.as_bytes().len() + 1) as *mut c_char };
    if m.is_null() {
        return m;
    }
    unsafe {
        libc::strcpy(m, c_str.as_ptr());
    };

    m
}

/// # Safety
/// This function is unsafe because it dereferences the `str_ptr` and `delim_str` pointers.
#[no_mangle]
pub unsafe extern "C" fn rume_strings_split(
    str_ptr: *const c_char,
    delim_str: *const c_char,
    behavior_ptr: *const libc::c_int,
) -> *mut *mut c_char {
    rume_strings_split_impl(str_ptr, delim_str, behavior_ptr)
}
