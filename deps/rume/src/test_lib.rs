#[cfg(test)]
mod test {
    use libc::free;

    use crate::{rume_get_init_str, rume_use_foo, Foo};

    #[test]
    fn test_rume_get_init_str() {
        let mut init_str: *mut std::os::raw::c_char = std::ptr::null_mut();

        let return_val = unsafe { rume_get_init_str(&mut init_str) };

        assert_eq!(return_val, 0);
        let result_str = unsafe { std::ffi::CStr::from_ptr(init_str) }
            .to_str()
            .unwrap();

        assert_eq!(result_str, "Some string from Rust",);

        unsafe { free(init_str as *mut std::ffi::c_void) };
    }

    #[test]
    fn test_rume_use_foo() {
        let str_ptr = unsafe { rume_use_foo(Foo::A([1.0; 2])) };

        assert!(!str_ptr.is_null());

        let str_val = unsafe { std::ffi::CStr::from_ptr(str_ptr) }
            .to_str()
            .unwrap_or("Invalid UTF-8 string");

        assert_eq!(str_val, "A([1.0, 1.0])");

        unsafe { free(str_ptr as *mut std::ffi::c_void) };
    }
}
