use std::io::{self, BufRead, BufStream, Write};
use std::net::{TcpStream, ToSocketAddrs};

use command::Command;

pub struct IrcConnection {
    buf: BufStream<TcpStream>,
}

impl IrcConnection {

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<IrcConnection> {
        let stream = try!(TcpStream::connect(addr));
        let buf = BufStream::new(stream);
        Ok(IrcConnection{
            buf: buf,
        })
    }

    pub fn send(&mut self, cmd: Command) -> io::Result<()> {
        try!(write!(self.buf, "{}\r\n", cmd));
        self.buf.flush()
    }

    pub fn read(&mut self) -> io::Result<String> {
        let mut line = String::new();
        try!(self.buf.read_line(&mut line));
        Ok(line)
    }

}
