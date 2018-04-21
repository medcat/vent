use std::os::unix::io::{RawFd, AsRawFd, FromRawFd};
use std::fs::File;
use std::io::{Read, Write};
use std::io::Result as IoResult;
use error::*;
use byteorder::{ReadBytesExt, WriteBytesExt, NativeEndian};

#[derive(Debug)]
pub struct Notify(File, u64);

impl Notify {
    pub fn read_value(&mut self) -> Result<u64> {
        self.read_u64::<NativeEndian>().chain_err(|| ErrorKind::NotifyReadError)
    }

    pub fn write_value(&mut self, value: u64) -> Result<()> {
        self.write_u64::<NativeEndian>(value).chain_err(|| ErrorKind::NotifyWriteError)
    }
}

impl From<(RawFd, u64)> for Notify {
    fn from(source: (RawFd, u64)) -> Notify {
        Notify(unsafe { File::from_raw_fd(source.0) }, source.1)
    }
}

impl Read for Notify {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.0.read(buf)
    }
}

impl AsRawFd for Notify {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl Write for Notify {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.0.flush()
    }
}
