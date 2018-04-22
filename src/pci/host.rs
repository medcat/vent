use std::cell::Cell;
use std::collections::HashMap;
use instance::{device, Instance, PortDirection};
use error::*;
use super::invalid;
use super::{Pci, Address as ConfigAddress};

/// A 32 bit register.  Bit 31 is an enable flag for determining when 
/// accesses to `CONFIG_DATA` should be translated to configuration cycles. Bits 
/// 23 through 16 allow the configuration software to choose a specific PCI bus 
/// in the system. Bits 15 through 11 select the specific device on the PCI Bus. 
/// Bits 10 through 8 choose a specific function in a device (if the device 
/// supports multiple functions). 
pub const CONFIG_ADDRESS_PORT: u16 = 0xCF8;

/// A 32 bit register, containing the data for the address port.
pub const CONFIG_DATA_PORT: u16 = 0xCFC;

struct Host {
    devices: HashMap<(u8, u8), Box<Pci>>,
    address: Cell<u32>
}

impl Host {
    pub fn new() -> Host {
        Host { devices: HashMap::new(), address: Cell::new(0) }
    }
}

impl device::Device for Host {
    fn initialize(&mut self, _instance: &mut Instance) -> Result<()> {
        Ok(())
    }

    fn port(&mut self, port: u16, direction: PortDirection, pointer: *mut u8, _size: usize) -> device::Action {
        match (port, direction) {
            (CONFIG_ADDRESS_PORT, PortDirection::In) => {
                unsafe { *(pointer as *mut u32) = self.address.get(); }
                device::Action::Stop
            },
            (CONFIG_ADDRESS_PORT, PortDirection::Out) => {
                unsafe { self.address.set(*(pointer as *const u32)); }
                device::Action::Stop
            },

            (CONFIG_DATA_PORT, PortDirection::In) => {
                let address = self.address.get().into();
                let sticky = unsafe { &mut *(pointer as *mut u32) };
                self.config_read(address, sticky)
            },

            (CONFIG_DATA_PORT, PortDirection::Out) => {
                let address = self.address.get().into();
                let sticky = unsafe { &mut *(pointer as *mut u32) };
                self.config_write(address, sticky)
            },

            _ => device::Action::Continue
        }
    }
}

impl Pci for Host {
    fn config_read(&mut self, address: ConfigAddress, pointer: &mut u32) -> device::Action {
        match address {
            ConfigAddress(0, 0, 0, r) => get_config(self, r, pointer),
            ConfigAddress(b, d, _, _) => {
                if let Some(device) = self.devices.get_mut(&(b, d)) {
                    device.config_read(address, pointer)
                } else {
                    invalid::Invalid.config_read(address, pointer)
                }
            }
        }
    }

    fn config_write(&mut self, address: ConfigAddress, pointer: &mut u32) -> device::Action {
        match address {
            ConfigAddress(0, 0, 0, _) => device::Action::Stop,
            ConfigAddress(b, d, _, _) => {
                if let Some(device) = self.devices.get_mut(&(b, d)) {
                    device.config_write(address, pointer)
                } else {
                    invalid::Invalid.config_write(address, pointer)
                }
            }
        }
    }
}

fn get_config(_h: &mut Host, register: u8, pointer: &mut u32) -> device::Action {
    match register {
        // 0x0001 will be our device id, 0x1af4 will be the vendor.  The vendor
        // is the virtio vendor, and the device id is chosen to be non-compliant.
        0x00 => *pointer = (0x0001u32 << 16) | (0x1af4u32),
        // class 0x06 (bridge), 0x00/0x00 (host bridge)
        0x08 => *pointer = 0x06u32 << 24,
        // does not support BIST.
        0x0C => *pointer = 0x01u32 << 16,
        0x10 => *pointer = 0b1100u32,
        _ => *pointer = 0
    }

    device::Action::Stop
}
