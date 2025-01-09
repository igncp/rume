#[cfg(test)]
mod test {
    use std::ffi::{c_char, c_void, CStr};

    use libc::free;

    use crate::{
        lib_impl::rume_strings_split_impl, rume_get_init_str, rume_use_foo, Foo,
        STRING_SPLIT_BEHAVIOR_SKIP_TOKEN,
    };

    #[test]
    fn test_rume_get_init_str() {
        let mut init_str: *mut c_char = std::ptr::null_mut();

        let return_val = unsafe { rume_get_init_str(&mut init_str) };

        assert_eq!(return_val, 0);
        let result_str = unsafe { CStr::from_ptr(init_str) }.to_str().unwrap();

        assert_eq!(result_str, "Some string from Rust",);

        unsafe { free(init_str as *mut c_void) };
    }

    #[test]
    fn test_rume_use_foo() {
        let str_ptr = unsafe { rume_use_foo(Foo::A([1.0; 2])) };

        assert!(!str_ptr.is_null());

        let str_val = unsafe { CStr::from_ptr(str_ptr) }
            .to_str()
            .unwrap_or("Invalid UTF-8 string");

        assert_eq!(str_val, "A([1.0, 1.0])");

        unsafe { free(str_ptr as *mut c_void) };
    }

    #[test]
    fn test_rume_strings_split_impl() {
        let base_str = "a,b,c";
        let base_cstr = std::ffi::CString::new(base_str).unwrap();
        let delim_str = ",";
        let delim_cstr = std::ffi::CString::new(delim_str).unwrap();
        let behavior_ptr = std::ptr::null();

        let vec_str_ptr =
            rume_strings_split_impl(base_cstr.as_ptr(), delim_cstr.as_ptr(), behavior_ptr);

        assert!(!vec_str_ptr.is_null());

        let mut vec_str = Vec::new();
        let mut i = 0;
        loop {
            let str_ptr = unsafe { *vec_str_ptr.add(i) };
            if str_ptr.is_null() {
                break;
            }

            let str_val = unsafe { CStr::from_ptr(str_ptr) }
                .to_str()
                .unwrap_or("Invalid UTF-8 string");

            vec_str.push(str_val);

            i += 1;
        }

        assert_eq!(vec_str, vec!["a", "b", "c"]);

        unsafe { free(vec_str_ptr as *mut c_void) };
    }

    #[test]
    fn test_rume_strings_split_impl_2() {
        let base_str = "ba";
        let base_cstr = std::ffi::CString::new(base_str).unwrap();
        let delim_str = " ";
        let delim_cstr = std::ffi::CString::new(delim_str).unwrap();

        let vec_str_ptr = rume_strings_split_impl(
            base_cstr.as_ptr(),
            delim_cstr.as_ptr(),
            &STRING_SPLIT_BEHAVIOR_SKIP_TOKEN,
        );

        assert!(!vec_str_ptr.is_null());

        let mut vec_str = Vec::new();
        let mut i = 0;
        loop {
            let str_ptr = unsafe { *vec_str_ptr.add(i) };
            if str_ptr.is_null() {
                break;
            }

            let str_val = unsafe { CStr::from_ptr(str_ptr) }
                .to_str()
                .unwrap_or("Invalid UTF-8 string");

            vec_str.push(str_val);

            i += 1;
        }

        assert_eq!(vec_str, vec!["ba"]);

        unsafe { free(vec_str_ptr as *mut c_void) };
    }
}
