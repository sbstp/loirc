use std::convert::AsRef;
use std::io::{self, BufRead, BufStream, Write};
use std::net::{TcpStream, ToSocketAddrs};

use message::{Message, ParseError};

pub struct IrcConnection<'a> {
    pub username: &'a str,
    pub realname: &'a str,
    pub nickname: &'a str,
    pub password: Option<&'a str>,
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
impl<'a> IrcConnection<'a> {

    pub fn new<'b, A: ToSocketAddrs>(addr: A,
                username: &'b str,
                realname: &'b str,
                nickname: &'b str,
                password: Option<&'b str>) -> io::Result<IrcConnection<'b>> {
        let stream = try!(TcpStream::connect(addr));
        let buf = BufStream::new(stream);

        let mut con = IrcConnection {
            username: username,
            realname: realname,
            nickname: nickname,
            password: password,
            buf: buf,
        };

        try!(con.pass());
        try!(con.nick());
        try!(con.user());

        Ok(con)
    }

    /// USER
    fn user(&mut self) -> io::Result<()> {
        let cmd = format!("USER {} 8 * :{}", self.username, self.realname);
        self.raw(cmd)
    }

    /// PASS
    fn pass(&mut self) -> io::Result<()> {
        match self.password {
            Some(password) => self.raw(format!("PASS {}", password)),
            None => Ok(())
        }

    }

    /// NICK
    fn nick(&mut self) -> io::Result<()> {
        let cmd = format!("NICK {}", self.nickname);
        self.raw(cmd)
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
