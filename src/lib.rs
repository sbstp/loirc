//! This library's goal is to offer a highly available IRC client with an easy to use API.
//! Automatic reconnection is built into the design. It uses a channel-like design, where
//! events are received from a `Reader` and commands are sent from one or many `Writer`.
//!
//! The `Writer` are thread safe and can be cheaply cloned.
//!
//! Here's a canonical example.
//!
//! ```no_run
//! // connect to freenode and use the default reconnection settings.
//! let (writer, reader) = irc::connect("irc.freenode.net:6667", None).unwrap();
//! writer.user("username", "realname");
//! writer.nick("nickname");
//! // Block until something happens.
//! for event in reader.iter() {
//!     match event {
//!         // Handle messages
//!         irc::Event::Message(msg) => {
//!             if msg.code == irc::Code::RplWelcome {
//!                 writer.join("#channel", None);
//!             }
//!         }
//!         // Handle other events, such as disconnects.
//!         _ => {}
//!     }
//! }
//! ```
extern crate time;

mod code;
mod connection;
mod message;
pub mod util;

pub use connection::{connect, Event, Error, Reader, ReconnectionSettings, Writer};
pub use code::Code;
pub use message::{ParseError, Message, Prefix, User};
