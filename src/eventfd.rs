use std::os::unix::io::{RawFd, AsRawFd, FromRawFd};
use std::io;
use std::result::Result as StdResult;
use std::fs::File;

use error::*;

use mio;
use futures;
use tokio;
use tokio::io::AsyncRead;

use byteorder::{ReadBytesExt, NativeEndian};

#[derive(Debug)]
pub struct EventFd(File);

impl EventFd {
    pub fn new() -> Result<EventFd> {
        use nix::sys::eventfd;
        let fd = eventfd::eventfd(0, eventfd::EfdFlags::EFD_NONBLOCK)
            .chain_err(|| ErrorKind::EventFdCreateError)?;
        Ok(unsafe { EventFd::from_raw_fd(fd) })
    }
}

impl AsRawFd for EventFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl FromRawFd for EventFd {
    unsafe fn from_raw_fd(raw: RawFd) -> EventFd {
        EventFd(File::from_raw_fd(raw))
    }
}

impl mio::event::Evented for EventFd {
    fn register(&self, poll: &mio::Poll, token: mio::Token, interest: mio::Ready, opts: mio::PollOpt) -> io::Result<()>
    {
        mio::unix::EventedFd(&self.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &mio::Poll, token: mio::Token, interest: mio::Ready, opts: mio::PollOpt) -> io::Result<()>
    {
        mio::unix::EventedFd(&self.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &mio::Poll) -> io::Result<()> {
        mio::unix::EventedFd(&self.as_raw_fd()).deregister(poll)
    }
}

impl AsRef<File> for EventFd {
    fn as_ref(&self) -> &File { &self.0 }
}

impl AsMut<File> for EventFd {
    fn as_mut(&mut self) -> &mut File { &mut self.0 }
}

impl io::Read for EventFd {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl io::Write for EventFd {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl tokio::io::AsyncRead for EventFd {}

impl tokio::io::AsyncWrite for EventFd {
    fn shutdown(&mut self) -> StdResult<tokio::prelude::Async<()>, tokio::io::Error> {
        Ok(().into())
    }
}

impl futures::Stream for EventFd {
    type Item = u64;
    type Error = Error;

    fn poll(&mut self) -> StdResult<futures::Async<Option<u64>>, Error> {
        let mut buf: [u8; 8] = [0; 8];
        match self.poll_read(&mut buf) {
            Ok(tokio::prelude::Async::Ready(_)) => {
                let mut cursor = io::Cursor::new(buf);
                let value = cursor.read_u64::<NativeEndian>()
                    .chain_err(|| ErrorKind::EventFdReadError)?;
                Ok(futures::Async::Ready(Some(value)))
            },
            Ok(tokio::prelude::Async::NotReady) =>
                Ok(futures::Async::NotReady),
            Err(e) => Err(Error::with_chain(e, ErrorKind::NotifyReadError))
        }
    }
}
