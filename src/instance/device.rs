use super::{Instance, PortDirection};
use error::*;

pub trait Device {
    fn initialize(&mut self, _instance: &mut Instance) -> Result<()> {
        Ok(())
    }

    fn port(&mut self, _port: u16, _direction: PortDirection, _pointer: *mut u8, _size: usize) -> Action {
        Action::Continue
    }

    fn mmio(&mut self, _addr: u64, _data: &mut [u8]) -> Action {
        Action::Continue
    }
}

#[derive(Debug)]
pub enum Action {
    Continue,
    Stop,
    Err(Error)
}

impl Into<Error> for Action {
    fn into(self) -> Error {
        match self {
            Action::Err(e) => e,
            _ => ErrorKind::ActionNotError.into(),
        }
    }
}
