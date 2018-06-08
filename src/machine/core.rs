use super::super::error::*;
use super::device;
use super::Machine;
use kvm;
use kvm::core::Pause;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn prepare(_machine: &Machine, _core: &mut kvm::Core) -> Result<()> {
    Ok(())
}

fn dump_core_data(core: &mut kvm::Core) {
    debug!("{:x?}", core.mp_state());
    debug!("regs: {:x?}", core.registers());
    debug!("sregs: {:x?}", core.special_registers());
    debug!(
        "run: {:x?}",
        <kvm::memory::Slab as AsRef<kvm::sys::Run>>::as_ref(&core.value)
    );
}

pub fn run(core: kvm::Core, devices: Vec<Arc<device::Device>>) -> thread::JoinHandle<()> {
    let core = Arc::new(Mutex::new(core));
    let mut reverse = HashMap::new();

    for device in &devices {
        for request in device.request() {
            reverse
                .entry(request)
                .or_insert_with(|| vec![])
                .push(device.clone());
        }
    }

    let tcore = core.clone();

    thread::spawn(move || loop {
        let mut core = tcore.lock().unwrap();
        let r = core.run();
        r.unwrap();

        match core.pause() {
            Pause::Shutdown => {
                error!("shutdown!");
                return;
            }
            Pause::Io {
                direction,
                size,
                port,
                count,
                data_offset,
            } => {
                for i in 0..count {
                    let start = data_offset as usize + (i as usize * size as usize);
                    let mut mem = vec![0; size as usize];
                    core.value.read_bytes(data_offset as usize, &mut mem);
                    let addr = kvm::core::IoAddress::Port(port as u64);
                    let action = kvm::core::IoAction(addr, direction, size as usize);

                    match reverse.get(&addr) {
                        Some(devices) => devices.iter().for_each(|device| {
                            device.handle(action, &mut mem);
                        }),
                        _ => {
                            // warn!("action: {:x?}: {:x?}", action, mem);
                        }
                    }

                    core.value.write_bytes(start, &mem);
                }
            }
            p => {
                error!("unknown pause reason {:x?}", p);
                dump_core_data(&mut core);
                return;
            }
        }

        core.clear()
    })
}
