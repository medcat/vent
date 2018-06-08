use super::Device;
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
    fn handle(&self, io: kvm::core::IoAction, memory: &mut [u8]) -> Option<()> {
        match io.direction() {
            kvm::core::IoDirection::Out => {
                warn!("e9handle({:x?}, {:x?})", io, memory);
                // let serr = ::std::io::stdout();
                // let mut stderr = serr.lock();
                // stderr.write_all(memory).unwrap();
                // stderr.flush().unwrap();
                Some(())
            }
            kvm::core::IoDirection::In => {
                for i in 0..memory.len() {
                    memory[i] = 0xff;
                }
                Some(())
            }
        }
    }
}
