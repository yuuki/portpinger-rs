#![crate_name = "portpinger"]
extern crate libc;

use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::mem;
use std::os::unix::io::AsRawFd;

pub fn tcp_ping<A: ToSocketAddrs>(addr: A) -> io::Result<()> {
    let stream = TcpStream::connect(addr)?;
    let mut linger: libc::linger = unsafe { mem::zeroed() };
    linger.l_onoff = 1 as libc::c_int;
    linger.l_linger = 0 as libc::c_int;

    let fd = stream.as_raw_fd();
    if unsafe {
        libc::setsockopt(fd,
                         libc::SOL_SOCKET,
                         libc::SO_LINGER,
                         (&linger as *const libc::linger) as *const libc::c_void,
                         mem::size_of::<libc::linger>() as u32)
    } == -1 {
        let err = io::Error::last_os_error();
        return Err(err);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
