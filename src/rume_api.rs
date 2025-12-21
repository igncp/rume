use std::ffi::c_int;

use crate::rume::Rume as RumeImpl;

pub struct Rume {
    inner: RumeImpl,
}

impl Default for Rume {
    fn default() -> Self {
        Self::new()
    }
}

impl Rume {
    pub fn new() -> Self {
        Rume {
            inner: RumeImpl::new(None),
        }
    }
    pub fn init(&mut self) -> c_int {
        match self.inner.init() {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }
}
