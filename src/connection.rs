use std::convert::AsRef;
use std::io::{self, BufRead, BufStream, Write};
use std::net::{TcpStream, ToSocketAddrs};

use message::{Message, ParseError};

pub struct IrcConnection {
    buf: BufStream<TcpStream>,
}

#[derive(Debug)]
pub enum IrcError {
    IoError(io::Error),
    ParseError(ParseError),
}

impl From<io::Error> for IrcError {
    fn from(err: io::Error) -> Self { IrcError::IoError(err) }
}

impl From<ParseError> for IrcError {
    fn from(err: ParseError) -> Self { IrcError::ParseError(err) }
}

/// Connection
impl IrcConnection {

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<IrcConnection> {
        let stream = try!(TcpStream::connect(addr));
        let buf = BufStream::new(stream);
        Ok(IrcConnection{
            buf: buf,
        })
    }

    /// USER
    pub fn user<S: AsRef<str>>(&mut self, username: S, realname: S) -> io::Result<()> {
        self.raw(format!("USER {} 8 * :{}", username.as_ref(), realname.as_ref()))
    }

    /// PASS
    pub fn pass<S: AsRef<str>>(&mut self, password: S) -> io::Result<()> {
        self.raw(format!("PASS {}", password.as_ref()))
    }

    /// NICK
    pub fn nick<S: AsRef<str>>(&mut self, nickname: S) -> io::Result<()> {
        self.raw(format!("NICK {}", nickname.as_ref()))
    }

    /// JOIN
    pub fn join<S: AsRef<str>>(&mut self, channels: S, password: Option<S>) -> io::Result<()> {
        match password {
            Some(password) => self.raw(format!("JOIN {} {}", channels.as_ref(), password.as_ref())),
            None => self.raw(format!("JOIN {}", channels.as_ref())),
        }
    }

    /// PART
    pub fn part<S: AsRef<str>>(&mut self, channels: S, message: Option<S>) -> io::Result<()> {
        match message {
            Some(message) => self.raw(format!("PART {} {}", channels.as_ref(), message.as_ref())),
            None => self.raw(format!("PART {}", channels.as_ref())),
        }
    }

    /// PRIVMSG
    pub fn privmsg<S: AsRef<str>>(&mut self, target: S, message: S) -> io::Result<()> {
        self.raw(format!("PRIVMSG {} :{}", target.as_ref(), message.as_ref()))
    }

    /// Send a raw message to the IRC server.
    /// Line endings are added by this method.
    pub fn raw<S: AsRef<str>>(&mut self, raw: S) -> io::Result<()> {
        try!(write!(self.buf, "{}\r\n", raw.as_ref()));
        self.buf.flush()
    }

    /// Get the next message from the IRC server.
    /// Blocks until a message is received.
    pub fn next(&mut self) -> Result<Message, IrcError> {
        let mut line = String::new();
        try!(self.buf.read_line(&mut line));
        Ok(try!(Message::parse(&line)))
    }

}
