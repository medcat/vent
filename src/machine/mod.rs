use super::configuration::MachineConfiguration;
use super::error::*;
use kvm;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

mod bios;
mod core;
mod device;
mod e9;
// mod gdt;

pub use self::e9::E9;

pub struct Machine {
    pub mach: kvm::Machine,
    cores: Vec<kvm::Core>,
    devices: Vec<Arc<device::Device>>,
}

// const MEMORY_32BIT_MAX: u64 = u32::max_value() as u64;
// const MEMORY_GAP_SIZE: u64 = 786 << 20;
const MEMORY_GAP_START: u64 = 0xc0000000;
const MEMORY_GAP_END: u64 = 0xffffffff;
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
        self.mach.create_irqchip()?;
        self.mach.set_tss_addr(None)?;
        self.mach.set_identity_map_addr(None)?;
        let adjusted = config.memory + MEMORY_RAM_START;

        if adjusted > MEMORY_GAP_START {
            let slab = self.mach.create_memory_slab(MEMORY_GAP_START as usize)?;
            self.mach.mount_memory_region(0, slab)?;
            let slab = self
                .mach
                .create_memory_slab((adjusted - MEMORY_GAP_START) as usize)?;
            self.mach.mount_memory_region(MEMORY_GAP_END, slab)?;
        } else {
            let slab = self.mach.create_memory_slab(adjusted as usize)?;
            self.mach.mount_memory_region(0, slab)?;
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
