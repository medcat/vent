use super::device::Device;
use kvm;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct E9(Option<u64>);

impl E9 {
    pub fn new(port: Option<u64>) -> E9 {
        E9(port)
    }
}

impl Device for E9 {
    fn request(&self) -> Vec<kvm::core::IoAddress> {
        vec![kvm::core::IoAddress::Port(self.0.unwrap_or(0xe9))]
    }
    fn handle(
        &self,
        _io: kvm::core::IoAddress,
        direction: kvm::core::IoDirection,
        memory: &mut [u8],
    ) {
        match direction {
            kvm::core::IoDirection::Out => {
                let serr = ::std::io::stderr();
                let mut stderr = serr.lock();
                stderr.write_all(memory).unwrap();
                stderr.flush().unwrap();
            }
            _ => {}
        }
    }
}
