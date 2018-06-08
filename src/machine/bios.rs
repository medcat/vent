/*
 * X86-32 Memory Map (typical)
 *                                        start      end
 * Real Mode Interrupt Vector Table       0x00000000 0x000003FF
 * BDA area                               0x00000400 0x000004FF
 * Conventional Low Memory                0x00000500 0x0009FBFF
 * EBDA area                              0x0009FC00 0x0009FFFF
 * VIDEO RAM                              0x000A0000 0x000BFFFF
 * VIDEO ROM (BIOS)                       0x000C0000 0x000C7FFF
 * ROMs & unus. space (mapped hw & misc)  0x000C8000 0x000EFFFF 160 KiB (typically)
 * Motherboard BIOS                       0x000F0000 0x000FFFFF
 * Extended Memory                        0x00100000 0xFEBFFFFF
 * Reserved (configs, ACPI, PnP, etc)     0xFEC00000 0xFFFFFFFF
 */

use super::super::error::*;
use super::Machine;
use super::MEMORY_GAP_END;
use std::fs::File;
use std::io::Read;
use std::path::Path;

static BIOS_PATH: &'static str = "bios.bin";

fn prepare_firmware(machine: &mut Machine) -> Result<()> {
    let path = Path::new(BIOS_PATH);
    let metadata = path
        .metadata()
        .chain_err(|| ErrorKind::InvalidFirmwareError("could not retrieve metadata of firmware"))?;
    if !metadata.is_file() {
        return Err(ErrorKind::InvalidFirmwareError("firmware is not a file").into());
    }

    let size = metadata.len();

    let mut file = File::open(path)
        .chain_err(|| ErrorKind::InvalidFirmwareError("could not open file for reading"))?;

    let mut bios = Vec::with_capacity(size as usize);
    file.read_to_end(&mut bios)
        .chain_err(|| ErrorKind::InvalidFirmwareError("could not read file"))?;

    let mslab = machine
        .mach
        .create_read_only_memory_region(MEMORY_GAP_END - size, size as usize)?;
    let mut slab = mslab.lock().unwrap();

    slab.write_bytes(0, &bios);

    Ok(())
}

pub fn prepare(machine: &mut Machine) -> Result<()> {
    prepare_firmware(machine)?;
    Ok(())
}
