#[derive(Debug)]
/// A borrowed variant of the message struct.
/// All the fields are borrowed.
/// This makes the API much nicer to use.
pub struct Message<'a> {
    /// Prefix
    pub prefix: Option<&'a str>,
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
        let mut prefix: Option<&str> = None;
        let mut command: Option<&str> = None;
        let mut args: Vec<&str> = Vec::new();
        let mut suffix: Option<&str> = None;

        // Look for a prefix
        if state.starts_with(":") {
            match state.find(" ") {
                None => return Err(ParseError::UnexpectedEnd),
                Some(idx) => {
                    prefix = Some(&state[1..idx]);
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
            prefix: self.prefix.as_ref().map(|s| s.to_string()),
            command: self.command.to_string(),
            args: args,
            suffix: self.suffix.as_ref().map(|s| s.to_string()),
        }
    }

}


#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
    EmptyCommand,
    EmptyMessage,
    UnexpectedEnd,
}

/// An owned variant of the Message struct.
/// All the field are owned.
/// This makes it easier to send messages to other threads.
#[derive(Clone, Debug)]
pub struct OwnedMessage {
    /// Prefix
    pub prefix: Option<String>,
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
            prefix: self.prefix.as_ref().map(|s| &s[..]),
            command: &self.command[..],
            args: args,
            suffix: self.suffix.as_ref().map(|s| &s[..]),
        }
    }

}

#[test]
fn test_full() {
    let res = Message::parse(":org.prefix.cool COMMAND arg1 arg2 arg3 :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.prefix, Some("org.prefix.cool"));
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
    assert_eq!(msg.prefix, Some("org.prefix.cool"));
    assert_eq!(msg.command, "COMMAND");
    assert_eq!(msg.args, vec!["arg1", "arg2", "arg3"]);
    assert_eq!(msg.suffix, None);
}

#[test]
fn test_no_args() {
    let res = Message::parse(":org.prefix.cool COMMAND :suffix is pretty cool yo");
    assert!(res.is_ok());
    let msg = res.ok().unwrap();
    assert_eq!(msg.prefix, Some("org.prefix.cool"));
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
