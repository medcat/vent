use std::os::unix::io::AsRawFd;
use eventfd::EventFd;
use kvm_sys as sys;
use error::*;
use super::Machine;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IoAddress {
        Port(u64),
        Memory(u64)
}

impl IoAddress {
    fn raw_address(&self) -> u64 {
        match self {
            &IoAddress::Port(v) => v,
            &IoAddress::Memory(v) => v
        }
    }

    fn raw_flags(&self) -> u32 {
        match self {
            &IoAddress::Port(_) => sys::KVM_IOEVENTFD_FLAG_PIO,
            _ => 0
        }
    }
}

#[derive(Debug)]
pub struct IoEventFd<'a> {
    machine: &'a Machine,
    event: EventFd,
    address: IoAddress,
    length: u32
}

impl<'a> IoEventFd<'a> {
    pub(super) fn new(machine: &'a Machine, address: IoAddress, length: u32) -> Result<IoEventFd> {
        let ioeventfd = IoEventFd { machine, event: EventFd::new()?, address, length };
        unsafe {
            sys::kvm_ioeventfd(machine.0, &ioeventfd.raw())
                .chain_err(|| ErrorKind::KvmMachineOperationError)
                .and_then(|v| if v >= 0 {
                        Ok(ioeventfd)
                    } else {
                        Err(ErrorKind::KvmMachineOperationError.into())
                    })
        }
    }

    fn raw(&self) -> sys::IoEventFd {
        sys::IoEventFd {
            datamatch: 0,
            addr: self.address.raw_address(),
            len: self.length,
            fd: self.event.as_raw_fd(),
            flags: self.address.raw_flags(),
            _pad: [0; 36]
        }
    }
}

impl<'a> Drop for IoEventFd<'a> {
    fn drop(&mut self) {
        let mut raw = self.raw();
        raw.flags &= sys::KVM_IOEVENTFD_FLAG_DEASSIGN;
        let _ = unsafe { sys::kvm_ioeventfd(self.machine.0, &raw) };
    }
}

impl<'a> AsRef<EventFd> for IoEventFd<'a> {
    fn as_ref(&self) -> &EventFd { &self.event }
}

impl<'a> AsMut<EventFd> for IoEventFd<'a> {
    fn as_mut(&mut self) -> &mut EventFd { &mut self.event }
}

unsafe impl<'a> Send for IoEventFd<'a> {}
