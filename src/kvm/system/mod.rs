use std::os::unix::io::{RawFd, IntoRawFd};
use std::fs::OpenOptions;
use nix;
use super::{sys, Machine, Error, ErrorKind, Result, ResultExt, CheckCapability, Capability};

#[derive(Debug)]
pub struct System(pub(super) RawFd);

impl System {
    pub fn new() -> Result<System> {
        OpenOptions::new().read(true).write(true).open("/dev/kvm")
            .map(|f| System(f.into_raw_fd()))
            .chain_err(|| ErrorKind::KvmSystemOpenError)
    }

    pub fn api_version(&mut self) -> Result<i32> {
        unsafe { sys::kvm_get_api_version(self.0) }
            .chain_err(|| ErrorKind::KvmSystemOperationError)
    }

    pub fn create_machine(&mut self) -> Result<Machine> {
        unsafe { sys::kvm_create_vm(self.0, 0).map(|v| Machine(v)) }
            .chain_err(|| ErrorKind::KvmSystemOperationError)
    }

    pub fn get_msr_indexes(&mut self) -> Result<Vec<u32>> {
        unsafe {
            use std::mem::{transmute, size_of};
            use nix::libc;
            // first, we ping it to figure out how much we need...
            let raw = &mut [0u32] as *mut [u32; 1];
            let blist: *mut sys::MsrList = transmute(raw);
            let _ = sys::kvm_get_msr_index_list(self.0, blist);
            let len = (*blist).nmsrs as usize;
            let list = libc::malloc(size_of::<sys::MsrList>() + len * size_of::<u32>()) as *mut sys::MsrList;
            if list.is_null() { bail!(ErrorKind::KvmSystemOperationError) }
            let result = sys::kvm_get_msr_index_list(self.0, list);

            match result {
                Ok(v) if v == 0 => {
                    let mut vec: Vec<u32> = Vec::with_capacity((*list).nmsrs as usize);
                    for i in 0..((*list).nmsrs as usize) {
                        vec.push((*list).indicies[i])
                    }

                    libc::free(list as *mut libc::c_void);
                    Ok(vec)
                },
                Ok(_) => {
                    libc::free(list as *mut libc::c_void);
                    Err(ErrorKind::KvmSystemOperationError.into())
                },
                Err(err) => {
                    libc::free(list as *mut libc::c_void);
                    Err(Error::with_chain(err, ErrorKind::KvmSystemOperationError))
                }
            }
        }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        nix::unistd::close(self.0).unwrap()
    }
}

impl CheckCapability for System {
    fn check_capability(&mut self, cap: Capability) -> Result<i32> {
        unsafe { sys::kvm_check_extension(self.0, cap.into()) }
            .chain_err(|| ErrorKind::KvmCapabilityCheckError)
    }
}

unsafe impl Send for System {}
