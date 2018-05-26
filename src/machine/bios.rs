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
use std::fs::File;
use std::io::Read;
use std::path::Path;

// const IVT_AREA: (u64, u64) = (0x0, 0x3ff);
// const BDA_AREA: (u64, u64) = (0x400, 0x4ff);
// const LOW_AREA: (u64, u64) = (0x500, 0x9fbff);
// const EBDA_AREA: (u64, u64) = (0x9fc00, 0x9ffff);
// const VIDEO_RAM: (u64, u64) = (0xa0000, 0xbffff);
// const VIDEO_ROM: (u64, u64) = (0xc0000, 0xc7fff);
// const BIOS_AREA: (u64, u64) = (0xf0000, 0xfffff);
const FIRM_AREA: (u64, u64) = (0xe0000, 0xfffff);

static BIOS_PATH: &'static str = "/usr/share/seabios/bios.bin";
// static BIOS_PATH: &'static str = "bios.bin";

fn area_size(area: (u64, u64)) -> u64 {
    area.1 - area.0 + 1
}

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

    if size > area_size(FIRM_AREA) {
        return Err(ErrorKind::InvalidFirmwareError(
            "firmware cannot fit in memory",
        ))?;
    }

    let mut bios = Vec::with_capacity(size as usize);
    file.read_to_end(&mut bios)
        .chain_err(|| ErrorKind::InvalidFirmwareError("could not read file"))?;

    // let slab = memory::Slab::from_file(file.into_raw_fd(), 0, size as usize)?;
    //
    // machine
    //     .mach
    //     .mount_memory_region(FIRM_AREA.1 - size + 1, slab.into())
    //     .chain_err(|| ErrorKind::InvalidFirmwareError("could not mount firmware"))?;

    let (mslab, offset) = machine.mach.locate(FIRM_AREA.1 - size).ok_or_else(|| {
        <ErrorKind as Into<Error>>::into(ErrorKind::InvalidFirmwareError("could not locate memory"))
    })?;

    let mut slab = mslab.lock().unwrap();
    let dest = &mut slab[(offset as usize)..((offset + size + 1) as usize)];

    unsafe {
        ::libc::memcpy(
            dest.as_mut_ptr() as *mut ::libc::c_void,
            (&bios[..]).as_ptr() as *const ::libc::c_void,
            dest.len(),
        )
    };

    Ok(())
}

// fn clear_region(machine: &mut Machine, region: (u64, u64)) -> Result<()> {
//     let (slab_lock, slab_offset) = machine
//         .mach
//         .locate(region.0)
//         .ok_or_else(|| ErrorKind::InvalidFirmwareError("could not clear region"))?;
//     let mut slab = slab_lock.lock().unwrap();
//     let region = &mut slab[(slab_offset as usize)..((slab_offset as usize))];
//
//     unsafe { ::libc::memset(region.as_mut_ptr() as *mut ::libc::c_void, 0, region.len()) };
//
//     Ok(())
// }

pub fn prepare(machine: &mut Machine) -> Result<()> {
    prepare_firmware(machine)?;
    Ok(())
}
