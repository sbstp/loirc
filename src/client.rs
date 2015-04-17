use std::io::{self, BufRead, BufStream, Write};
use std::net::{TcpStream, ToSocketAddrs};

use message::{Message, OwnedMessage, ParseError};

pub struct Client {
    stream: BufStream<TcpStream>,
}

impl Client {

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Client> {
        let stream = try!(TcpStream::connect(addr));
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

    pub fn read(&mut self) -> Result<OwnedMessage, ReadError> {
        let mut line = String::new();
        try!(self.stream.read_line(&mut line));
        let msg = try!(Message::parse(&line[..]));
        Ok(msg.to_owned())
    }

}

#[derive(Debug)]
pub enum ReadError {
    IoError(io::Error),
    ParseError(ParseError),
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::IoError(err)
    }
}

impl From<ParseError> for ReadError {
    fn from(err: ParseError) -> ReadError {
        ReadError::ParseError(err)
    }
}
