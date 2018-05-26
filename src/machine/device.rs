use kvm;
use std::fmt::Debug;

pub trait Device: Debug + Send + Sync {
    fn request(&self) -> Vec<kvm::core::IoAddress>;
    fn handle(
        &self,
        io: kvm::core::IoAddress,
        direction: kvm::core::IoDirection,
        memory: &mut [u8],
    );
}

impl<T: Device> Device for Box<T> {
    fn request(&self) -> Vec<kvm::core::IoAddress> {
        self.as_ref().request()
    }

    fn handle(
        &self,
        io: kvm::core::IoAddress,
        direction: kvm::core::IoDirection,
        memory: &mut [u8],
    ) {
        self.as_ref().handle(io, direction, memory)
    }
}
