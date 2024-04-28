use thiserror::Error;
use fly_machines_gen::Error;

#[derive(Error, Debug)]
pub enum FlyError {
}

pub type Result<T> = core::result::Result<T, FlyError>;