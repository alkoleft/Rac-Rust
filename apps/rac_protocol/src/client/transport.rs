use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::rac_wire::{read_frame, write_frame, Frame};

#[derive(Debug)]
pub struct RacTransport {
    stream: TcpStream,
}

impl RacTransport {
    pub fn connect(
        addr: &str,
        connect_timeout: Duration,
        read_timeout: Duration,
        write_timeout: Duration,
    ) -> io::Result<Self> {
        let mut addrs = addr.to_socket_addrs()?;
        let socket_addr = addrs
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid address"))?;
        let stream = TcpStream::connect_timeout(&socket_addr, connect_timeout)?;
        stream.set_read_timeout(Some(read_timeout))?;
        stream.set_write_timeout(Some(write_timeout))?;
        stream.set_nodelay(true)?;
        Ok(Self { stream })
    }

    pub fn write_raw(&mut self, payload: &[u8]) -> io::Result<()> {
        use io::Write;
        self.stream.write_all(payload)
    }

    pub fn write_frame(&mut self, opcode: u8, payload: &[u8]) -> io::Result<()> {
        write_frame(&mut self.stream, opcode, payload)
    }

    pub fn read_frame(&mut self) -> io::Result<Frame> {
        read_frame(&mut self.stream)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        use io::Write;
        self.stream.flush()
    }
}
