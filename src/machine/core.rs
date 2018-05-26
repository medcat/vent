use super::super::error::*;
use super::device;
use super::Machine;
use kvm;
use kvm::core::Pause;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

const BIOS_SELECTOR: u16 = 0xf000;
const BIOS_IP: u64 = 0xfff0;
const BIOS_SP: u64 = 0;

const DEFAULT_CPU_REGISTERS: kvm::core::Registers = kvm::core::Registers {
    rax: 0,
    rbx: 0,
    rcx: 0,
    rdx: 0,
    rsi: 0,
    rdi: 0,
    rsp: BIOS_SP,
    rbp: BIOS_SP,
    r8: 0,
    r9: 0,
    r10: 0,
    r11: 0,
    r12: 0,
    r13: 0,
    r14: 0,
    r15: 0,
    rip: BIOS_IP,
    rflags: 2,
};

pub fn prepare(_machine: &Machine, core: &mut kvm::Core) -> Result<()> {
    core.registers()?;
    core.set_registers(DEFAULT_CPU_REGISTERS)?;

    let mut special_registers = core.special_registers()?;

    special_registers.cs.selector = BIOS_SELECTOR;
    special_registers.cs.base = (BIOS_SELECTOR as u64) << 4;
    special_registers.ss.selector = BIOS_SELECTOR;
    special_registers.ss.base = (BIOS_SELECTOR as u64) << 4;
    special_registers.ds.selector = BIOS_SELECTOR;
    special_registers.ds.base = (BIOS_SELECTOR as u64) << 4;
    special_registers.es.selector = BIOS_SELECTOR;
    special_registers.es.base = (BIOS_SELECTOR as u64) << 4;
    special_registers.fs.selector = BIOS_SELECTOR;
    special_registers.fs.base = (BIOS_SELECTOR as u64) << 4;
    special_registers.gs.selector = BIOS_SELECTOR;
    special_registers.gs.base = (BIOS_SELECTOR as u64) << 4;

    if core.id == 0 {
        special_registers.cr0 = 1;
    }

    core.set_special_registers(special_registers)?;

    Ok(())
}

fn dump_core_data(core: &kvm::Core) {
    for i in 0..16 {
        print!("{:#05x} ", i);

        for e in 0..16 {
            print!("{:02x} ", core.value[(i * 16) + e]);
        }

        println!("");
    }
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
        warn!(
            "translated location for 0xfff0: {:?}",
            core.translate(0xfff0)
        );
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
                    let start = data_offset as usize + i as usize * size as usize;
                    let mem = &mut core.value[start..(start + (size as usize))];
                    let addr = kvm::core::IoAddress::Port(port as u64);
                    info!(
                        "received data on port Port({:#x})/{:?}/{:x}",
                        port, direction, size
                    );
                    match reverse.get_mut(&addr) {
                        Some(devices) => devices
                            .iter_mut()
                            .for_each(|device| device.handle(addr, direction, mem)),
                        None => {
                            warn!("received notification of data on {:?}/{:?}, but no devices requested it", addr, direction);
                        }
                    }
                }
            }
            p => {
                error!("unknown pause reason {:?}", p);
                dump_core_data(&core);
                return;
            }
        }
    })
}
