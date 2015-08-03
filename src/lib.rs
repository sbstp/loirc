//! This library's goal is to offer a highly available IRC client with an easy to use API.
//! Automatic reconnection is built into the design. It uses a channel-like design, where
//! events are received from a `Reader` and commands are sent from one or many `Writer`.
//!
//! The `Writer` are thread safe and can be cheaply cloned.
//!
//! Here's a canonical example.
//!
//! ```no_run
//! use loirc::{connect, Code, Event};
//!
//! // connect to freenode and use the default reconnection settings.
//! let (writer, reader) = connect("irc.freenode.net:6667", Default::default()).unwrap();
//! writer.user("username", "realname");
//! writer.nick("nickname");
//! // Block until something happens.
//! for event in reader.iter() {
//!     match event {
//!         // Handle messages
//!        Event::Message(msg) => {
//!             if msg.code == Code::RplWelcome {
//!                 writer.join("#channel", None);
//!             }
//!         }
//!         // Handle other events, such as disconnects.
//!         _ => {}
//!     }
//! }
//! ```
#![deny(missing_docs)]

extern crate time;

mod activity_monitor;
mod code;
mod connection;
mod message;

pub use time::Duration;

pub use activity_monitor::{ActivityMonitor, MonitorSettings};
pub use connection::{connect, Event, Error, Reader, ReconnectionSettings, Writer};
pub use code::Code;
pub use message::{ParseError, Message, Prefix, User};
