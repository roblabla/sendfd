use std::os::unix::net::{UnixStream, UnixDatagram};
use std::io::{Result, Error};
use std::os::raw::c_int;
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;

extern {
    fn sendfd(socket: c_int, fd: c_int) -> c_int;
    fn recvfd(socket: c_int) -> c_int;
}

/// This trait is the safe abstraction that allows sending file descriptors over
/// a Unix Socket
pub trait UnixSendFd {
    /// Send a File Descriptor over a socket.
    fn sendfd(&self, fd: RawFd) -> Result<()>;
    /// Receive a File Descriptor over a socket.
    fn recvfd(&self) -> Result<RawFd>;
}

impl UnixSendFd for UnixStream {
    fn sendfd(&self, fd: RawFd) -> Result<()> {
        let err = unsafe { sendfd(self.as_raw_fd(), fd) };
        if err < 0 {
            Err(Error::from_raw_os_error(err))
        } else {
            Ok(())
        }
    }

    fn recvfd(&self) -> Result<RawFd> {
        let fd = unsafe { recvfd(self.as_raw_fd()) };
        if fd < 0 {
            Err(Error::from_raw_os_error(fd))
        } else {
            Ok(fd)
        }
    }
}

impl UnixSendFd for UnixDatagram {
    fn sendfd(&self, fd: RawFd) -> Result<()> {
        let err = unsafe { sendfd(self.as_raw_fd(), fd) };
        if err < 0 {
            Err(Error::from_raw_os_error(err))
        } else {
            Ok(())
        }
    }

    fn recvfd(&self) -> Result<RawFd> {
        let fd = unsafe { recvfd(self.as_raw_fd()) };
        if fd < 0 {
            Err(Error::from_raw_os_error(fd))
        } else {
            Ok(fd)
        }
    }
}

#[cfg(test)]
mod tests {
    use std;
    use std::os::unix::net::{UnixStream, UnixDatagram};
    use std::os::unix::io::{FromRawFd, AsRawFd};
    use std::io::{Read, Write};
    use UnixSendFd;
    #[test]
    fn it_works_on_datagram() {
        let (left, right) = UnixDatagram::pair().unwrap();
        let (left2, right2) = UnixDatagram::pair().unwrap();
        std::thread::spawn(move || {
            let right3 = unsafe { UnixDatagram::from_raw_fd(right.recvfd().unwrap()) };
            right3.send(b"test").unwrap();
        });
        left.sendfd(right2.as_raw_fd()).unwrap();
        let mut buf = [0; 15];
        let size = left2.recv(&mut buf).unwrap();
        assert_eq!(&buf[..size], b"test");
    }
    #[test]
    fn it_works_on_stream() {
        let (left, right) = UnixStream::pair().unwrap();
        let (mut left2, right2) = UnixStream::pair().unwrap();
        std::thread::spawn(move || {
            let mut right3 = unsafe { UnixStream::from_raw_fd(right.recvfd().unwrap()) };
            right3.write(b"test").unwrap();
        });
        left.sendfd(right2.as_raw_fd()).unwrap();
        let mut buf = [0; 15];
        let size = left2.read(&mut buf).unwrap();
        assert_eq!(&buf[..size], b"test");
    }
}
