use std::fs::File;
use std::os::unix::io::AsRawFd;
use nix;
use error::*;

pub use kvm_sys::UserspaceMemoryRegion as SysUserspaceMemoryRegion;

#[derive(Debug)]
pub struct UserspaceMemoryRegion(pub(super) SysUserspaceMemoryRegion);

impl UserspaceMemoryRegion {
    pub fn file(file: &File, slot: u32, addr: u64) -> Result<UserspaceMemoryRegion> {
        use nix::sys::mman;
        let meta = file.metadata().chain_err(|| ErrorKind::MemoryMapError)?;
        let len = meta.len();

        let data = unsafe {
            let protflag = mman::ProtFlags::PROT_READ | mman::ProtFlags::PROT_WRITE;
            let mapflag = mman::MapFlags::MAP_FILE | mman::MapFlags::MAP_SHARED;
            mman::mmap(0 as *mut nix::libc::c_void, len as usize, protflag, mapflag, file.as_raw_fd(), 0)
                .chain_err(|| ErrorKind::MemoryMapError)?
        };

        Ok(UserspaceMemoryRegion(SysUserspaceMemoryRegion {
            slot, flags: 0,
            guest_phys_addr: addr as u64,
            /// in bytes.
            memory_size: len,
            /// the start of th userspace allocated memory.
            userspace_addr: data as u64
        }))
    }

    pub fn anon(size: u64, slot: u32, addr: u64) -> Result<UserspaceMemoryRegion> {
      use nix::sys::mman;

      let data = unsafe {
        let protflag = mman::ProtFlags::PROT_READ | mman::ProtFlags::PROT_WRITE;
        let mapflag = mman::MapFlags::MAP_ANON | mman::MapFlags::MAP_SHARED;
        mman::mmap(0 as *mut nix::libc::c_void, size as usize, protflag, mapflag, 0, 0)
          .chain_err(|| ErrorKind::MemoryMapError)?
      };

      Ok(UserspaceMemoryRegion(SysUserspaceMemoryRegion {
        slot, flags: 0,
        guest_phys_addr: addr as u64,
        memory_size: size,
        userspace_addr: data as u64
      }))
    }
}

impl Drop for UserspaceMemoryRegion {
    fn drop(&mut self) {
        use nix::sys::mman;
        unsafe {
            mman::munmap(self.0.userspace_addr as *mut nix::libc::c_void, self.0.memory_size as usize).unwrap();
        }
    }
}

impl<T> AsRef<T> for UserspaceMemoryRegion {
  fn as_ref(&self) -> &T {
    unsafe { &*(self.0.userspace_addr as *mut T) }
  }
}

impl<T> AsMut<T> for UserspaceMemoryRegion {
  fn as_mut(&mut self) -> &mut T {
    unsafe { &mut *(self.0.userspace_addr as *mut T) }
  }
}
