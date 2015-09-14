use std::io::{self, BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use time::Duration;

use message::{Message, ParseError};

/// This is the comprehensive set of events that can occur.
#[derive(Debug)]
pub enum Event {
    /// Connection was manually closed. The string is the reason.
    Closed(&'static str),
    /// Connection has dropped.
    Disconnected,
    /// Message from the IRC server.
    Message(Message),
    /// Error parsing a message from the server.
    ///
    /// This can probably be ignored, and it shouldn't ever happen, really.
    /// If you catch this you should probably open an issue on GitHub.
    ParseError(ParseError),
    /// Connection was sucessfully restored.
    Reconnected,
    /// Attempting to restore connection.
    Reconnecting,
    /// An error occured trying to restore the connection.
    ///
    /// This is normal in poor network conditions. It might take
    /// a few attempts before the connection can be restored.
    ReconnectionError(io::Error),
}

/// This the receiving end of a `mpsc` channel.
///
/// If is closed/dropped, the connection will also be dropped,
/// as there isn't anyone listening to the events anymore.
pub type Reader = Receiver<Event>;

/// Errors produced by the Writer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Connection is already closed.
    AlreadyClosed,
    /// Connection is already disconnected.
    AlreadyDisconnected,
    /// Connection was manually closed.
    Closed,
    /// Connection was dropped.
    ///
    /// A reconnection might be in process.
    Disconnected,
}

enum StreamStatus {
    // The stream was closed manually.
    Closed,
    // The stream is connected.
    Connected(TcpStream),
    // The stream is disconnected, an attempt to reconnect will be made.
    Disconnected,
}

/// Used to send messages to the IRC server.
///
/// This object is thread safe. You can clone it and send the clones to other
/// threads. You can write from multiple threads without any issue. Internally,
/// it uses `Arc` and `Mutex`.
#[derive(Clone)]
pub struct Writer {
    stream: Arc<Mutex<StreamStatus>>,
}

impl Writer {

    fn new(stream: TcpStream) -> Writer {
        Writer {
            stream: Arc::new(Mutex::new(StreamStatus::Connected(stream)))
        }
    }

    fn set_connected(&self, stream: TcpStream) {
        *self.stream.lock().unwrap() = StreamStatus::Connected(stream);
    }

    fn set_disconnected(&self) {
        *self.stream.lock().unwrap() = StreamStatus::Disconnected;
    }

    /// Drop the connection and trigger the reconnection process.
    ///
    /// There might be a reconnection attempt, based on your settings.
    /// This should be used if you want the connection to be re-created.
    /// This is not the preferred way of shutting down the connection
    /// for good. Use `close` for this.
    pub fn disconnect(&self) -> Result<(), Error> {
        let mut status = self.stream.lock().unwrap();

        match *status {
            StreamStatus::Closed => {
                return Err(Error::Closed);
            }
            StreamStatus::Connected(ref mut stream) => {
                let _ = stream.shutdown(Shutdown::Both);
            }
            StreamStatus::Disconnected => {
                return Err(Error::AlreadyDisconnected);
            }
        }

        *status = StreamStatus::Disconnected;
        Ok(())
    }

    /// Check if the connection was manually closed.
    pub fn is_closed(&self) -> bool {
        match *self.stream.lock().unwrap() {
            StreamStatus::Closed => true,
            _ => false,
        }
    }

    /// Close the connection and stop listening for messages.
    ///
    /// There will not be any reconnection attempt.
    /// An error will be returned if the connection is already closed.
    pub fn close(&self) -> Result<(), Error> {
        let mut status = self.stream.lock().unwrap();

        match *status {
            StreamStatus::Closed => {
                return Err(Error::AlreadyClosed);
            }
            StreamStatus::Connected(ref mut stream) => {
                let _ = stream.shutdown(Shutdown::Both);
            }
            _ => {}
        }

        *status = StreamStatus::Closed;
        Ok(())
    }

    /// Send a raw string to the IRC server.
    ///
    /// A new line will be not be added, so make sure that you include it.
    /// An error will be returned if the client is disconnected.
    pub fn raw<S: AsRef<str>>(&self, data: S) -> Result<(), Error> {
        let mut status = self.stream.lock().unwrap();
        let mut failed = false;

        match *status {
            StreamStatus::Closed => {
                return Err(Error::Closed);
            }
            StreamStatus::Connected(ref mut stream) => {
                // Try to write to the stream.
                if stream.write(data.as_ref().as_bytes()).is_err() {
                    // The write failed, shutdown the connection.
                    let _ = stream.shutdown(Shutdown::Both);
                    failed = true;
                }
            }
            StreamStatus::Disconnected => {
                return Err(Error::Disconnected);
            }
        }

        if failed {
            // The write failed, change the status.
            *status = StreamStatus::Disconnected;
            Err(Error::Disconnected)
        } else {
            // The write did not fail.
            Ok(())
        }
    }

}

impl Into<Event> for Result<Message, ParseError> {

    fn into(self) -> Event {
        match self {
            Ok(msg) => Event::Message(msg),
            Err(err) => Event::ParseError(err),
        }
    }

}

/// These settings tell the reconnection process how to behave.
///
/// Default is implemented for this type, with fairly sensible settings.
/// See the Default trait implementation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReconnectionSettings {
    /// Don't try to reconnect after failure.
    DoNotReconnect,
    /// Reconnect
    Reconnect {
        /// After trying this amount of times, it will stop trying.
        ///
        /// A value of 0 means infinite attempts.
        max_attempts: u32,
        /// Wait time between two attempts to reconnect in milliseconds.
        ///
        /// Note that if the computer's network is still unavailable, the connect
        /// call might block for about a minute until it fails. Somtimes, it fails
        /// instantly because it cannot resolve the hostname. You should probably
        /// leave at least a second of delay, so that it doesn't loop really fast
        /// while getting hostname resolution errors. You can watch the stream of
        /// errors via the ReconnectionError event.
        delay_between_attempts: Duration,
        /// Wait time after disconnection, before trying to reconnect.
        delay_after_disconnect: Duration,
    }
}

/// Default settings are provided for this enum.
///
/// They are:
///
/// `max_attempts` = 10
///
/// `delay_between_attempts` = 5 seconds
///
/// `delay_after_disconnect` = 60 seconds
impl Default for ReconnectionSettings {

    fn default() -> ReconnectionSettings {
        ReconnectionSettings::Reconnect {
            max_attempts: 10,
            delay_between_attempts: Duration::seconds(5),
            delay_after_disconnect: Duration::seconds(60),
        }
    }

}

fn reconnect(address: &str, handle: &Writer) -> io::Result<(BufReader<TcpStream>)> {
    let stream = try!(TcpStream::connect(address));
    let reader = BufReader::new(try!(stream.try_clone()));
    handle.set_connected(stream);
    Ok((reader))
}

fn reader_thread(address: String, mut reader: BufReader<TcpStream>,
                                   event_sender: Sender<Event>, handle: Writer,
                                   reco_settings: ReconnectionSettings) {
    'read: loop {
        let mut line = String::new();
        let res = reader.read_line(&mut line);

        // If there's an error or a zero length read, we should check to reconnect or exit.
        // If the size is 0, it means that the socket was shutdown.
        if res.is_err() || res.unwrap() == 0 {
            // If the stream has the closed status, the stream was manually closed.
            if handle.is_closed() {
                let _ = event_sender.send(Event::Closed("manually closed"));
                break;
            } else {
                // The stream was not closed manually, see what we should do.

                // Set the disconnected status on the writer.
                handle.set_disconnected();

                if event_sender.send(Event::Disconnected).is_err() {
                    break;
                }

                // Grab the reconnection settings or break the loop if no reconnection is desired.
                let (max_attempts, delay_between_attempts, delay_after_disconnect) = match reco_settings {
                    ReconnectionSettings::DoNotReconnect => {
                        let _ = event_sender.send(Event::Closed("do not reconnect"));
                        break;
                    }
                    ReconnectionSettings::Reconnect{ max_attempts,
                                                     delay_between_attempts,
                                                     delay_after_disconnect } => {
                        (max_attempts, delay_between_attempts, delay_after_disconnect)
                    }
                };

                thread::sleep_ms(delay_after_disconnect.num_milliseconds() as u32);

                let mut attempts = 0u32;

                // Loop until reconnection is successful.
                'reconnect: loop {

                    // If max_attempts is zero, it means an infinite amount of attempts.
                    if max_attempts > 0 {
                        attempts += 1;
                        if attempts > max_attempts {
                            let _ = event_sender.send(Event::Closed("max attempts reached"));
                            break 'read;
                        }
                    }

                    if event_sender.send(Event::Reconnecting).is_err() {
                        break 'read;
                    }

                    // Try to reconnect.
                    match reconnect(&address, &handle) {
                        // Sucess, send event, and update reader.
                        Ok(new_reader) => {
                            reader = new_reader;
                            if event_sender.send(Event::Reconnected).is_err() {
                                break 'read;
                            }

                            break 'reconnect;
                        }
                        // Error, send event.
                        Err(err) => {
                            if event_sender.send(Event::ReconnectionError(err)).is_err() {
                                break 'read;
                            }
                        }
                    }
                    // sleep until we try to reconnect again
                    thread::sleep_ms(delay_between_attempts.num_milliseconds() as u32);
                }
            }
        } else {
            // Size is bigger than 0, try to parse the message. Send the result in the channel.
            if event_sender.send(Message::parse(&line).into()).is_err() {
                break;
            }
        }
    }

    // If we exited from a break (failed to send message through channel), we might not
    // have closed the stream cleanly. Do so if necessary.
    if !handle.is_closed() {
        let _ = handle.close();
    }
}

/// Create a connection to the given address.
///
/// A `Writer`/`Reader` pair is returned. If the connection fails,
/// an error is returned.
///
/// If you don't want to reconnect, use `ReconnectionSettings::DoNotReconnect`.
pub fn connect<A: AsRef<str>>(address: A, reco_settings: ReconnectionSettings) -> io::Result<(Writer, Reader)> {

    let stream = try!(TcpStream::connect(address.as_ref()));
    let reader = BufReader::new(try!(stream.try_clone()));

    let (event_sender, event_reader) = mpsc::channel::<Event>();

    let writer = Writer::new(stream);
    // The reader thread needs a handle to modify the status.
    let reader_handle = writer.clone();

    let address_clone = address.as_ref().to_owned();
    thread::spawn(move || {
        reader_thread(address_clone, reader, event_sender, reader_handle, reco_settings);
    });

    Ok((writer, event_reader))
}
