use std::io::{self, BufRead, BufStream, Write};
use std::net::{TcpStream, ToSocketAddrs};

use message::{Message, OwnedMessage, ParseError};

pub struct Client {
    stream: TcpStream,
    buf: BufStream<TcpStream>,
}

impl Client {

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Client> {
        let stream = try!(TcpStream::connect(addr));
        let buf = BufStream::new(try!(stream.try_clone()));
        Ok(Client{
            stream: stream,
            buf: buf,
        })
    }

    pub fn send_raw<A: AsRef<str>>(&mut self, line: A) -> io::Result<()> {
        try!(write!(self.buf, "{}\r\n", line.as_ref()));
        self.buf.flush()
    }

    pub fn user(&mut self, username: &str, realname: &str) -> io::Result<()> {
        self.send_raw(format!("USER {} 8 * :{}", username, realname))
    }

    pub fn nick(&mut self, nickname: &str) -> io::Result<()> {
        self.send_raw(format!("NICK {}", nickname))
    }

    pub fn join(&mut self, channel: &str) -> io::Result<()> {
        self.send_raw(format!("JOIN {}", channel))
    }

    pub fn read(&mut self) -> Result<OwnedMessage, ReadError> {
        let mut line = String::new();
        try!(self.buf.read_line(&mut line));
        let msg = try!(Message::parse(&line[..]));
        Ok(msg.to_owned())
    }

    pub fn try_clone(&self) -> io::Result<Client> {
        let stream = try!(self.stream.try_clone());
        let buf = BufStream::new(try!(self.stream.try_clone()));
        Ok(Client{
            stream: stream,
            buf: buf,
        })
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
