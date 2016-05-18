extern crate bytes;
extern crate mio;
extern crate slab;

mod code;
mod connection;
mod message;
mod util;

use std::io;
use std::time::Duration;

pub use code::Code;
pub use message::Message;
pub use connection::{Connection, ConnectionManager, ConnectionStatus, Writer};

pub trait RawEvents: Send {

    fn closed(&mut self) {}

    fn connect(&mut self, w: Writer) {}

    fn disconnect(&mut self) {}

    fn message(&mut self, w: Writer, msg: Message) {}

}


/// These settings tell the monitor how to behave.
///
/// They allow you to configure the amount of time between the steps.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MonitorSettings {
    /// Amount of time since the last activity.
    ///
    /// When the amount of time since the last activity gets higher than this value,
    /// it will trigger a ping request. If there is not a lot of activity, this means
    /// that the server will be pinged everytime this duration expires.
    pub activity_timeout: Duration,
    /// Amount of time to wait for a ping reply.
    ///
    /// When the amount of time since the ping was sent gets higher than this value,
    /// and that no activity occured, assume the connection was dropped and trigger
    /// and a disconnect.
    pub ping_timeout: Duration,
}


/// Default values are provided for the settings.
///
/// They are:
///
/// `activity_timeout` = 60 seconds
///
/// `ping_timeout` = 15 seconds
impl Default for MonitorSettings {

    fn default() -> MonitorSettings {
        MonitorSettings {
            activity_timeout: Duration::from_secs(60),
            ping_timeout: Duration::from_secs(15),
        }
    }

}
