use super::configuration::MachineConfiguration;
use super::device;
use super::error::*;
use kvm;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

mod acpi;
mod bios;
mod core;

pub struct Machine {
    pub mach: kvm::Machine,
    cores: Vec<kvm::Core>,
    devices: Vec<Arc<device::Device>>,
}

// const MEMORY_GAP_START: u64 = 0xc0000000;
const MEMORY_GAP_START: u64 = 0xff000000;
const MEMORY_GAP_END: u64 = 0xffffffff + 1;
const MEMORY_RAM_START: u64 = 0x00100000;

impl Machine {
    pub fn new(mach: kvm::Machine) -> Result<Machine> {
        Ok(Machine {
            mach,
            cores: vec![],
            devices: vec![],
        })
    }

    pub fn push(&mut self, device: Arc<device::Device>) -> Result<()> {
        self.devices.push(device);
        Ok(())
    }

    pub fn prepare(&mut self, config: &MachineConfiguration) -> Result<()> {
        info!("preparing machine...");
        device::prepare(self)?;
        self.create_irqchip()?;
        self.create_pit()?;
        self.set_tss_addr(None)?;
        self.set_identity_map_addr(None)?;
        let adjusted = config.memory + MEMORY_RAM_START;

        if adjusted > MEMORY_GAP_START {
            self.create_memory_region(0, MEMORY_GAP_START as usize)?;
            self.create_memory_region(MEMORY_GAP_END, (adjusted - MEMORY_GAP_START) as usize)?;
        } else {
            self.create_memory_region(0, adjusted as usize)?;
        }

        let mut cores = vec![];
        (0..config.cores)
            .try_for_each(|id| self.mach.create_core(id).map(|core| cores.push(core)))?;
        cores
            .iter_mut()
            .try_for_each(|core| core::prepare(self, core))?;
        for core in cores {
            self.cores.push(core);
        }

        bios::prepare(self)?;

        Ok(())
    }

    pub fn run(self) -> () {
        let cores = self.cores;
        let devices = self.devices;

        let joins = cores
            .into_iter()
            .map(|core| {
                let locals = devices.iter().cloned().collect::<Vec<_>>();
                core::run(core, locals)
            })
            .collect::<Vec<_>>();

        for join in joins {
            join.join().unwrap();
        }
    }
}

impl Deref for Machine {
    type Target = kvm::Machine;

    fn deref(&self) -> &kvm::Machine {
        &self.mach
    }
}

impl DerefMut for Machine {
    fn deref_mut(&mut self) -> &mut kvm::Machine {
        &mut self.mach
    }
}
