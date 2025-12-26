use crate::strings::{split, SplitBehavior};
use crate::Foo;
use libc::c_int;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::ptr::null_mut;

pub const STRING_SPLIT_BEHAVIOR_KEEP_TOKEN_: libc::c_int = 1;
pub const STRING_SPLIT_BEHAVIOR_SKIP_TOKEN_: libc::c_int = 2;

const RETURNED_STRING: &str = "Some string from Rust";

// https://github.com/mozilla/cbindgen/tree/master/tests/rust

pub fn rume_get_init_str_impl(desc: *mut *mut c_char) -> i32 {
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

impl From<libc::c_int> for SplitBehavior {
    fn from(val: libc::c_int) -> Self {
        match val {
            STRING_SPLIT_BEHAVIOR_KEEP_TOKEN_ => SplitBehavior::KeepToken,
            STRING_SPLIT_BEHAVIOR_SKIP_TOKEN_ => SplitBehavior::SkipToken,
            _ => panic!("Invalid value for SplitBehavior"),
        }
    }
}

impl From<SplitBehavior> for libc::c_int {
    fn from(val: SplitBehavior) -> Self {
        match val {
            SplitBehavior::KeepToken => STRING_SPLIT_BEHAVIOR_KEEP_TOKEN_,
            SplitBehavior::SkipToken => STRING_SPLIT_BEHAVIOR_SKIP_TOKEN_,
        }
    }
}

pub fn rume_strings_split_impl(
    str_ptr: *const c_char,
    delim_str: *const c_char,
    behavior_ptr: c_int,
) -> *mut *mut c_char {
    let behavior = if behavior_ptr == 0 {
        None
    } else {
        Some(behavior_ptr.into())
    };

    let str = unsafe { CStr::from_ptr(str_ptr) }
        .to_str()
        .expect("Expecting valid UTF-8 string");
    let delim = unsafe { CStr::from_ptr(delim_str) }
        .to_str()
        .expect("Expecting valid UTF-8 string");
    let strings_vec = split(str, delim, behavior);

    let strings_vec_ptr = unsafe {
        libc::malloc(strings_vec.len() * mem::size_of::<*mut c_char>() + 1) as *mut *mut c_char
    };

    if strings_vec_ptr.is_null() {
        return null_mut();
    }

    for (i, s) in strings_vec.iter().enumerate() {
        let c_str = CString::new(s.as_str()).expect("Expecting we can allocate a CString");
        let m = unsafe { libc::malloc(c_str.as_bytes().len() + 1) as *mut c_char };

        if m.is_null() {
            return null_mut();
        }

        unsafe {
            *strings_vec_ptr.add(i) = m;
            libc::strcpy(*strings_vec_ptr.add(i), c_str.as_ptr());
        }
    }

    unsafe { *strings_vec_ptr.add(strings_vec.len()) = null_mut() };

    strings_vec_ptr
}

pub fn rume_use_foo_impl(test_param: Foo) -> *mut c_char {
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
