#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
    EmptyCommand,
    EmptyMessage,
    UnexpectedEnd,
}

/// An owned variant of the Message struct.
/// All the field are owned.
/// This makes it easier to send messages to other threads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    /// Prefix
    pub prefix: Option<Prefix>,
    /// Command/Reply
    pub command: String,
    /// Arguments
    pub args: Vec<String>,
    /// Suffix
    pub suffix: Option<String>,
}

impl Message {

    pub fn parse(line: &str) -> Result<Message, ParseError> {
        if line.len() == 0 || line.trim().len() == 0 {
            return Err(ParseError::EmptyMessage);
        }

        let mut state = line.trim_right_matches("\r\n");
        let mut prefix: Option<Prefix> = None;
        let mut command: Option<String> = None;
        let mut args: Vec<String> = Vec::new();
        let mut suffix: Option<String> = None;

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
                    command = Some(state[..].to_string());
                    state = &state[state.len()..];
                }
            }
            Some(idx) => {
                command = Some(state[..idx].to_string());
                state = &state[idx + 1..];
            }
        }

        // Look for arguments and the suffix
        if state.len() > 0 {
            loop {
                if state.starts_with(":") {
                    suffix = Some(state[1..].to_string());
                    break;
                } else {
                    match state.find(" ") {
                        None => {
                            args.push(state[..].to_string());
                            break;
                        }
                        Some(idx) => {
                            args.push(state[..idx].to_string());
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
}

fn parse_prefix(prefix: &str) -> Option<Prefix> {
    match prefix.find("!") {
        None => Some(Prefix::Server(prefix.to_string())),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Prefix {
    User(User),
    Server(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    nick: String,
    user: String,
    host: String,
}

impl User {

    pub fn new(nick: &str, user: &str, host: &str) -> User {
        User {
            nick: nick.to_string(),
            user: user.to_string(),
            host: host.to_string(),
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
    assert_eq!(msg.suffix, Some("suffix is pretty cool yo".to_string()));
}

#[test]
fn test_no_prefix() {
    let res = Message::parse("COMMAND arg1 arg2 arg3 :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.prefix, None);
    assert_eq!(msg.command, "COMMAND");
    assert_eq!(msg.args, vec!["arg1", "arg2", "arg3"]);
    assert_eq!(msg.suffix, Some("suffix is pretty cool yo".to_string()));
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
    assert_eq!(msg.suffix, Some("suffix is pretty cool yo".to_string()));
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
    assert_eq!(msg.prefix, Some(Prefix::Server("irc.freenode.net".to_string())));
}

#[test]
fn test_prefix_user() {
    let res = Message::parse(":bob!bob@bob.com COMMAND :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.prefix, Some(Prefix::User(User::new("bob", "bob", "bob.com"))));
}
