use super::deployer::Deployer;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Default)]
pub struct Service {
    _deployer: Deployer,
}

lazy_static! {
    pub static ref SERVICE: Mutex<Box<Service>> = Mutex::new(Box::new(Service::default()));
}

impl Service {
    pub fn instance() -> &'static SERVICE {
        &SERVICE
    }

    pub fn deployer(&self) -> &Deployer {
        &self._deployer
    }

    pub fn deployer_mut(&mut self) -> &mut Deployer {
        &mut self._deployer
    }
}
