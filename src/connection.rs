use std::convert::AsRef;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

use message::{Message, ParseError};

pub struct IrcConnection {
    pub nickname: String,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
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

    pub fn new<A: ToSocketAddrs>(addr: A,
                username: &str,
                realname: &str,
                nickname: &str,
                password: Option<&str>) -> io::Result<IrcConnection> {
        let stream = try!(TcpStream::connect(addr));

        let mut con = IrcConnection {
            nickname: nickname.into(),
            reader: BufReader::new(try!(stream.try_clone())),
            writer: BufWriter::new(try!(stream.try_clone())),
        };

        if let Some(password) = password {
            try!(con.pass(password));
        }
        // TODO nickname failure
        try!(con.nick(nickname));
        try!(con.user(username, realname));

        Ok(con)
    }

    /// USER
    fn user(&mut self, username: &str, realname: &str) -> io::Result<()> {
        let cmd = format!("USER {} 8 * :{}", username, realname);
        self.raw(cmd)
    }

    /// PASS
    fn pass(&mut self, password: &str) -> io::Result<()> {
        let cmd = format!("PASS {}", password);
        self.raw(cmd)
    }

    /// NICK
    fn nick(&mut self, nickname: &str) -> io::Result<()> {
        let cmd = format!("NICK {}", nickname);
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

    /// PONG
    pub fn pong<S: AsRef<str>>(&mut self, payload: S) -> io::Result<()> {
        self.raw(format!("PONG :{}", payload.as_ref()))
    }

    /// Send a raw message to the IRC server.
    /// Line endings are added by this method.
    pub fn raw<S: AsRef<str>>(&mut self, raw: S) -> io::Result<()> {
        try!(write!(self.writer, "{}\r\n", raw.as_ref()));
        self.writer.flush()
    }

    /// Get the next message from the IRC server.
    /// Blocks until a message is received.
    pub fn next(&mut self) -> Result<Message, IrcError> {
        let mut line = String::new();
        try!(self.reader.read_line(&mut line));
        Ok(try!(Message::parse(&line)))
    }

}
