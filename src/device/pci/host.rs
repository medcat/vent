use super::super::Device;
use super::{Address, Pci};
use byteorder::{ByteOrder, LittleEndian};
use kvm::core::{IoAction, IoAddress};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct Host(u8, AtomicUsize, HashMap<usize, Arc<Pci>>);

pub static CONFIG_ADDRESS: IoAddress = IoAddress::Port(0xcf8);
pub static CONFIG_DATA: IoAddress = IoAddress::Port(0xcfc);

// #[cfg_attr(rustfmt, rustfmt_skip)]
// pub static DEFAULT_HOST_CONFIG: &[u8] = &[
//     // a  a+1   a+2   a+3
//     0xf4, 0x1a, 0x01, 0x00, // 0x00
//     0x07, 0x00, 0x00, 0x00, // 0x04
//     0x00, 0x00, 0x00, 0x06, // 0x08
//     0x00, 0x00, 0x01, 0x00, // 0x0c
//     0x00, 0x00, 0x00, 0x00, // 0x10, BAR0
//     0x00, 0x00, 0x00, 0x00, // 0x14, BAR1
//     0x00, 0x00, 0x00, 0x00, // 0x18
//     0x00, 0x00, 0x00, 0x00, // 0x1c
//     0x00, 0x00, 0x00, 0x00, // 0x20
//     0x00, 0x00, 0x00, 0x00, // 0x24
//     0x00, 0x00, 0x00, 0x00, // 0x28
//     0x00, 0x00, 0x00, 0x00, // 0x2c
//     0x00, 0x00, 0x00, 0x00, // 0x30
//     0x00, 0x00, 0x00, 0x00, // 0x34
//     0x00, 0x00, 0x00, 0x00, // 0x38
//     0x00, 0x00, 0x00, 0x00, // 0x3c
//     0x00, 0x00, 0x00, 0x00, // 0x40
//     0x00, 0x00, 0x00, 0x00, // 0x44

// ];

impl Host {
    pub fn new<V: IntoIterator<Item = Arc<Pci>>>(bridge: Option<u8>, pcis: V) -> Host {
        let mut devices = HashMap::new();
        let bridge = bridge.unwrap_or(0);

        for (i, pci) in pcis.into_iter().enumerate() {
            devices.insert(i, pci);
        }

        Host(bridge, 0usize.into(), devices)
    }

    fn lookup(&self, device: u8) -> Option<&Pci> {
        self.2.get(&(device as usize)).map(|v| v.as_ref() as &Pci)
    }
}

impl Device for Host {
    fn request(&self) -> Vec<IoAddress> {
        vec![CONFIG_ADDRESS, CONFIG_DATA]
    }

    fn handle(&self, io: IoAction, memory: &mut [u8]) -> Option<()> {
        // info!("{:x?}/{:x?}", io, memory);
        if io == CONFIG_ADDRESS.outw() {
            self.1
                .store(LittleEndian::read_u32(memory) as usize, Ordering::SeqCst);
            Some(())
        } else if io == CONFIG_DATA.inw() {
            let addr = Address::from(self.1.load(Ordering::SeqCst) as u32);

            match addr {
                Some(address) if address.bridge() == self.0 => {
                    match self.lookup(address.device()) {
                        Some(device) => {
                            let value = device.config_read(address).unwrap_or(0);
                            LittleEndian::write_u32(memory, value);
                        }

                        None => {
                            for i in 0..4 {
                                memory[i] = 0xff;
                            }
                        }
                    }
                }
                _ => {}
            }

            Some(())
        } else if io == CONFIG_DATA.outw() {
            let addr = Address::from(self.1.load(Ordering::SeqCst) as u32);

            match addr {
                Some(address) if address.bridge() == self.0 => {
                    match self.lookup(address.device()) {
                        Some(device) => {
                            let value = LittleEndian::read_u32(memory);
                            warn!("config request for {:x?} successful: {:x?}", address, value);
                            device.config_write(address, value);
                        }
                        None => {}
                    }
                }

                _ => {}
            }
            Some(())
        } else {
            None
        }
    }
}
