use crate::{rume::Rume, rume_api_c::RumeC};

pub(super) fn return_result_helper<T, E>(result: Result<T, E>) -> i32 {
    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub(super) fn extract_rume_instance(instance: *mut RumeC) -> Option<&'static mut Rume> {
    if instance.is_null() {
        return None;
    }
    let instance_val = unsafe { &*instance };
    if instance_val.inner.is_null() {
        return None;
    }
    let rume_impl: &mut Rume = unsafe { &mut *(instance_val.inner as *mut Rume) };
    Some(rume_impl)
}
