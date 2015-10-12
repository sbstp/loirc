//! This library's goal is to offer a highly available IRC client with an easy to use API.
//! Automatic reconnection is built into the design. It uses a channel-like design, where
//! events are received from a `Reader` and commands are sent from one or many `Writer`.
//!
//! The `Writer` are thread safe and can be cheaply cloned.
//!
//! Here's a canonical example.
//!
//! ```no_run
//! extern crate encoding;
//! extern crate loirc;
//!
//! use encoding::all::UTF_8;
//! use loirc::{connect, Code, Event};
//!
//! fn main() {
//!     // connect to freenode and use the default reconnection settings.
//!     let (writer, reader) = connect("irc.freenode.net:6667", Default::default(), UTF_8).unwrap();
//!     writer.raw(format!("USER {} 8 * :{}\n", "username", "realname"));
//!     writer.raw(format!("NICK {}\n", "nickname"));
//!     // Block until something happens.
//!     for event in reader.iter() {
//!         match event {
//!             // Handle messages
//!             Event::Message(msg) => {
//!                 if msg.code == Code::RplWelcome {
//!                     writer.raw(format!("JOIN {}\n", "#channel"));
//!                 }
//!             }
//!             // Handle other events, such as disconnects.
//!             _ => {}
//!         }
//!     }
//! }
//! ```
#![deny(missing_docs)]
extern crate encoding;
extern crate time;

mod activity_monitor;
mod code;
mod connection;
mod message;

pub use activity_monitor::{ActivityMonitor, MonitorSettings};
pub use connection::{connect, Event, Error, Reader, ReconnectionSettings, Writer};
pub use code::Code;
pub use message::{ParseError, Message, Prefix, PrefixUser};
