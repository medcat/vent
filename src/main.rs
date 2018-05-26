extern crate kvm;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate tokio;
extern crate uuid;
// #[macro_use]
// extern crate bitflags;
#[macro_use]
extern crate log;
extern crate byteorder;
extern crate env_logger;
extern crate libc;

use kvm::capability::{Capability, CapabilityKind};
use std::error::Error;
use std::sync::Arc;

mod configuration;
mod error;
mod machine;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            error!("error: {}", e);

            if let Some(cause) = e.cause() {
                error!("cause: {}", cause);
            }
        }
    }
}

fn run() -> Result<(), error::Error> {
    env_logger::init();
    let mut system = kvm::System::new()?;
    assert_eq!(system.api_version()?, 12);
    system.check_capability(CapabilityKind::MemorySlotCount)?;
    let mut machine = machine::Machine::new(system.create_machine(0)?)?;
    machine.check_capability(CapabilityKind::MemorySlotCount)?;
    let config = configuration::MachineConfiguration {
        name: "".to_owned(),
        uuid: None,
        cores: 1,
        memory: 0x1000000,
    };
    machine.push(Arc::new(machine::E9::new(None)))?;
    machine.prepare(&config)?;
    machine.run();
    Ok(())
}
