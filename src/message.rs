#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
    EmptyCommand,
    EmptyMessage,
    UnexpectedEnd,
}

/// A borrowed variant of the message struct.
/// All the fields are borrowed.
/// This makes the API much nicer to use.
#[derive(Debug, Eq, PartialEq)]
pub struct Message<'a> {
    /// Prefix
    pub prefix: Option<Prefix<'a>>,
    /// Command/Reply
    pub command: &'a str,
    /// Arguments
    pub args: Vec<&'a str>,
    /// Suffix
    pub suffix: Option<&'a str>,
}

impl<'a> Message<'a> {

    pub fn parse<'b>(line: &'b str) -> Result<Message<'b>, ParseError> {
        if line.len() == 0 || line.trim().len() == 0 {
            return Err(ParseError::EmptyMessage);
        }

        let mut state = line.trim_right_matches("\r\n");
        let mut prefix: Option<Prefix> = None;
        let mut command: Option<&str> = None;
        let mut args: Vec<&str> = Vec::new();
        let mut suffix: Option<&str> = None;

        // Look for a prefix
        if state.starts_with(":") {
            match state.find(" ") {
                None => return Err(ParseError::UnexpectedEnd),
                Some(idx) => {
                    prefix = parse_prefix(&state[1..idx]);
                    state = &state[idx + 1..];
                }
            }
        }

        // Look for the command/reply
        match state.find(" ") {
            None => {
                if state.len() == 0 {
                    return Err(ParseError::EmptyMessage);
                } else {
                    command = Some(&state[..]);
                    state = &state[state.len()..];
                }
            }
            Some(idx) => {
                command = Some(&state[..idx]);
                state = &state[idx + 1..];
            }
        }

        // Look for arguments and the suffix
        if state.len() > 0 {
            loop {
                if state.starts_with(":") {
                    suffix = Some(&state[1..]);
                    break;
                } else {
                    match state.find(" ") {
                        None => {
                            args.push(&state[..]);
                            break;
                        }
                        Some(idx) => {
                            args.push(&state[..idx]);
                            state = &state[idx + 1..];
                        }
                    }
                }
            }
        }

        let cmd = match command {
            None => return Err(ParseError::EmptyCommand),
            Some(cmd) => cmd,
        };

        Ok(Message {
            prefix: prefix,
            command: cmd,
            args: args,
            suffix: suffix,
        })
    }

    pub fn to_owned(&self) -> OwnedMessage {
        let mut args: Vec<String> = Vec::new();

        for arg in self.args.iter() {
            args.push(arg.to_string());
        }

        OwnedMessage {
            prefix: self.prefix.as_ref().map(|s| s.to_owned()),
            command: self.command.to_string(),
            args: args,
            suffix: self.suffix.as_ref().map(|s| s.to_string()),
        }
    }

}

fn parse_prefix(prefix: &str) -> Option<Prefix> {
    match prefix.find("!") {
        None => Some(Prefix::Server(prefix)),
        Some(excpos) => {
            let nick = &prefix[..excpos];
            let rest = &prefix[excpos + 1..];
            match rest.find("@") {
                None => return None,
                Some(atpos) => {
                    let user = &rest[..atpos];
                    let host = &rest[atpos + 1..];
                    return Some(Prefix::User(User::new(nick, user, host)));
                }
            }
        }
    }
}

/// An owned variant of the Message struct.
/// All the field are owned.
/// This makes it easier to send messages to other threads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedMessage {
    /// Prefix
    pub prefix: Option<OwnedPrefix>,
    /// Command/Reply
    pub command: String,
    /// Arguments
    pub args: Vec<String>,
    /// Suffix
    pub suffix: Option<String>,
}

impl OwnedMessage {

    pub fn borrow<'a>(&'a self) -> Message<'a> {
        let mut args = Vec::new();

        for arg in self.args.iter() {
            args.push(&arg[..]);
        }

        Message {
            prefix: self.prefix.as_ref().map(|s| s.borrow()),
            command: &self.command[..],
            args: args,
            suffix: self.suffix.as_ref().map(|s| &s[..]),
        }
    }

}

#[derive(Debug, Eq, PartialEq)]
pub enum Prefix<'a> {
    User(User<'a>),
    Server(&'a str),
}

impl<'a> Prefix<'a> {

    pub fn to_owned(&self) -> OwnedPrefix {
        match *self {
            Prefix::User(ref user) => OwnedPrefix::User(user.to_owned()),
            Prefix::Server(ref serv) => OwnedPrefix::Server(serv.to_string()),
        }
    }

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnedPrefix {
    User(OwnedUser),
    Server(String),
}

impl OwnedPrefix {

    pub fn borrow(&self) -> Prefix {
        match *self {
            OwnedPrefix::User(ref user) => Prefix::User(user.borrow()),
            OwnedPrefix::Server(ref serv) => Prefix::Server(&serv[..]),
        }
    }

}

#[derive(Debug, Eq, PartialEq)]
pub struct User<'a> {
    nick: &'a str,
    user: &'a str,
    host: &'a str,
}

impl<'a> User<'a> {

    pub fn new<'b>(nick: &'b str, user: &'b str, host: &'b str) -> User<'b> {
        User {
            nick: nick,
            user: user,
            host: host,
        }
    }

    pub fn to_owned(&self) -> OwnedUser {
        OwnedUser {
            nick: self.nick.to_string(),
            user: self.user.to_string(),
            host: self.host.to_string(),
        }
    }

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedUser {
    nick: String,
    user: String,
    host: String,
}

impl OwnedUser {

    pub fn new(nick: String, user: String, host: String) -> OwnedUser {
        OwnedUser {
            nick: nick,
            user: user,
            host: host,
        }
    }

    pub fn borrow(&self) -> User {
        User {
            nick: &self.nick[..],
            user: &self.user[..],
            host: &self.host[..],
        }
    }

}

#[test]
fn test_full() {
    let res = Message::parse(":org.prefix.cool COMMAND arg1 arg2 arg3 :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.command, "COMMAND");
    assert_eq!(msg.args, vec!["arg1", "arg2", "arg3"]);
    assert_eq!(msg.suffix, Some("suffix is pretty cool yo"));
}

#[test]
fn test_no_prefix() {
    let res = Message::parse("COMMAND arg1 arg2 arg3 :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.prefix, None);
    assert_eq!(msg.command, "COMMAND");
    assert_eq!(msg.args, vec!["arg1", "arg2", "arg3"]);
    assert_eq!(msg.suffix, Some("suffix is pretty cool yo"));
}

#[test]
fn test_no_suffix() {
    let res = Message::parse(":org.prefix.cool COMMAND arg1 arg2 arg3");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.command, "COMMAND");
    assert_eq!(msg.args, vec!["arg1", "arg2", "arg3"]);
    assert_eq!(msg.suffix, None);
}

#[test]
fn test_no_args() {
    let res = Message::parse(":org.prefix.cool COMMAND :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.command, "COMMAND");
    assert_eq!(msg.args.len(), 0);
    assert_eq!(msg.suffix, Some("suffix is pretty cool yo"));
}

#[test]
fn test_only_command() {
    let res = Message::parse("COMMAND");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.prefix, None);
    assert_eq!(msg.command, "COMMAND");
    assert_eq!(msg.args.len(), 0);
    assert_eq!(msg.suffix, None);
}

#[test]
fn test_empty_message() {
    let res = Message::parse("");
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(err == ParseError::EmptyMessage);
}

#[test]
fn test_empty_message_trim() {
    let res = Message::parse("    ");
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(err == ParseError::EmptyMessage);
}

#[test]
fn test_only_prefix() {
    let res = Message::parse(":org.prefix.cool");
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(err == ParseError::UnexpectedEnd);
}

#[test]
fn test_prefix_none() {
    let res = Message::parse("COMMAND :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert!(msg.prefix == None);
}

#[test]
fn test_prefix_server() {
    let res = Message::parse(":irc.freenode.net COMMAND :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.prefix, Some(Prefix::Server("irc.freenode.net")));
}

#[test]
fn test_prefix_user() {
    let res = Message::parse(":bob!bob@bob.com COMMAND :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.prefix, Some(Prefix::User(User::new("bob", "bob", "bob.com"))));
}
