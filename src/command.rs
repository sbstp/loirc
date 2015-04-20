use std::fmt;

use code::Code;
use self::Command::*;

/// https://tools.ietf.org/html/rfc2812
/// Section 3 and 4
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    /// password
    Pass(String),
    /// nickname
    Nick(String),
    /// user, mode, realname
    User(String, String, String),
    /// nickname/channel, mode
    Mode(String, Option<String>),
    /// message
    Quit(Option<String>),
    /// channels, key
    Join(String, Option<String>),
    /// channels, message
    Part(String, Option<String>),
    /// channel, topic
    Topic(String, Option<String>),
    /// channel, nickname, message
    Kick(String, String, Option<String>),
    /// target, text
    Privmsg(String, String),
    /// target, text
    Notice(String, String),
    /// server1, server2
    Ping(Option<String>, Option<String>),
    /// server1, server2
    Pong(Option<String>, Option<String>),
    /// anything
    Raw(String),
}

impl fmt::Display for Command {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pass(ref pass) => write!(f, "{} {}", Code::Pass, pass),
            Nick(ref nick) => write!(f, "{} {}", Code::Nick, nick),
            User(ref user, ref mode, ref realname) => {
                write!(f, "{} {} {} * :{}", Code::User, user, mode, realname)
            }
            Mode(ref target, ref mode) => {
                write!(f, "{} {}{}", Code::Mode, target, opt(mode))
            }
            Quit(ref msg) => write!(f, "{}{}", Code::Quit, opt(msg)),
            Join(ref channels, ref pass) => {
                write!(f, "{} {}{}", Code::Join, channels, opt(pass))
            }
            Part(ref channels, ref msg) => {
                write!(f, "{} {}{}", Code::Part, channels, opts(msg))
            }
            Topic(ref channel, ref msg) => {
                write!(f, "{} {}{}", Code::Topic, channel, opts(msg))
            }
            Kick(ref channel, ref nick, ref msg) => {
                write!(f, "{} {} {}{}", Code::Kick, channel, nick, opts(msg))
            }
            Privmsg(ref target, ref text) => {
                write!(f, "{} {} :{}", Code::Privmsg, target, text)
            }
            Notice(ref target, ref text) => {
                write!(f, "{} {} :{}", Code::Notice, target, text)
            }
            Ping(ref s1, ref s2) => {
                write!(f, "{}{}{}", Code::Ping, opt(s1), opt(s2))
            }
            Pong(ref s1, ref s2) => {
                write!(f, "{}{}{}", Code::Pong, opt(s1), opt(s2))
            }
            Raw(ref any) => write!(f, "{}", any),
        }
    }

}

// Optional data
fn opt(o: &Option<String>) -> String {
    match *o {
        None => "".to_string(),
        Some(ref v) => {
            let mut s = String::new();
            s.push_str(" ");
            s.push_str(v);
            s
        }
    }
}

// Optionnal suffix data
fn opts(o: &Option<String>) -> String {
    match *o {
        None => "".to_string(),
        Some(ref v) => {
            let mut s = String::new();
            s.push_str(" :");
            s.push_str(v);
            s
        }
    }
}

#[test]
fn test_mode() {
    assert_eq!(
        format!("{}", Mode("user".to_string(), Some("+i".to_string()))),
        "MODE user +i".to_string()
    );
    assert_eq!(
        format!("{}", Mode("user".to_string(), None)),
        "MODE user".to_string()
    );
}

#[test]
fn test_part() {
    assert_eq!(
        format!("{}", Part("#c".to_string(), Some("cya nerds".to_string()))),
        "PART #c :cya nerds".to_string()
    );
    assert_eq!(
        format!("{}", Part("#c".to_string(), None)),
        "PART #c".to_string()
    );
}
