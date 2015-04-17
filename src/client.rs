use std::io::{self, BufRead, BufStream, Write};
use std::net::{TcpStream, ToSocketAddrs};

use message::{Message, ParseError};

pub struct Client {
    stream: BufStream<TcpStream>,
}

impl Client {

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Client> {
        let mut stream = try!(TcpStream::connect(addr));
        Ok(Client{
            stream: BufStream::new(stream),
        })
    }

    fn send(&mut self, line: String) -> io::Result<()> {
        try!(write!(self.stream, "{}\r\n", line));
        self.stream.flush()
    }

    pub fn user(&mut self, username: &str, realname: &str) -> io::Result<()> {
        self.send(format!("USER {} 8 * :{}", username, realname))
    }

    pub fn nick(&mut self, nickname: &str) -> io::Result<()> {
        self.send(format!("NICK {}", nickname))
    }

    pub fn join(&mut self, channel: &str) -> io::Result<()> {
        self.send(format!("JOIN {}", channel))
    }

    pub fn get(&mut self) -> io::Result<String> {
        let mut line = String::new();
        try!(self.stream.read_line(&mut line));
        Ok(line)
    }

}
