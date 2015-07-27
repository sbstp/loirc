use std::io::{self, BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use message::{Message, ParseError};

/// This is the comprehensive set of events that can occur.
#[derive(Debug)]
pub enum Event {
    /// Connection was manually closed.
    Closed,
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
    /// You might want to implement some kind of heuristic that
    /// closes the connection after a while.
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
    /// Connection was manually closed.
    Closed,
    /// Connection was dropped.
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

/// Used to send messages to the IrcServer.
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

    fn set_stream(&self, stream: TcpStream) {
        *self.stream.lock().unwrap() = StreamStatus::Connected(stream);
    }

    fn disconnect(&self) {
        *self.stream.lock().unwrap() = StreamStatus::Disconnected;
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
    #[allow(unused_must_use)]
    pub fn close(&self) -> Result<(), Error> {
        let mut status = self.stream.lock().unwrap();

        match *status {
            StreamStatus::Closed => {
                return Err(Error::AlreadyClosed);
            }
            StreamStatus::Connected(ref mut stream) => {
                stream.shutdown(Shutdown::Both);
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
    #[allow(unused_must_use)]
    pub fn raw(&self, data: String) -> Result<(), Error> {
        let mut status = self.stream.lock().unwrap();
        let mut failed = false;

        match *status {
            StreamStatus::Closed => {
                return Err(Error::Closed);
            }
            StreamStatus::Connected(ref mut stream) => {
                // Try to write to the stream.
                if stream.write(data.as_bytes()).is_err() {
                    // The write failed, shutdown the connection.
                    stream.shutdown(Shutdown::Both);
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

    /// NICK command.
    pub fn nick(&self, nickname: &str) -> Result<(), Error> {
        self.raw(format!("NICK {}\n", nickname))
    }

    /// USER command.
    pub fn user(&self, username: &str, realname: &str) -> Result<(), Error> {
        self.raw(format!("USER {} 8 * :{}\n", username, realname))
    }

    /// PING command.
    pub fn ping(&self, server: &str) -> Result<(), Error> {
        self.raw(format!("PING {}", server))
    }

    /// PONG command.
    pub fn pong(&self, server: &str) -> Result<(), Error> {
        self.raw(format!("PONG {}", server))
    }

    /// PRIVMSG command.
    pub fn privmsg(&self, target: &str, text: &str) -> Result<(), Error> {
        self.raw(format!("PRIVMSG {} :{}", target, text))
    }

    /// JOIN command.
    pub fn join(&self, channel: &str, password: Option<&str>) -> Result<(), Error> {
        match password {
            None => self.raw(format!("JOIN {}", channel)),
            Some(password) => self.raw(format!("JOIN {} {}", channel, password)),
        }
    }

    /// PART command.
    pub fn part(&self, channel: &str, message: Option<&str>) -> Result<(), Error> {
        match message {
            None => self.raw(format!("PART {}", channel)),
            Some(message) => self.raw(format!("PART {} :{}", channel, message)),
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

fn reconnect<A: ToSocketAddrs>(address: &A, handle: &Writer) -> io::Result<(BufReader<TcpStream>)> {
    let stream = try!(TcpStream::connect(address));
    let reader = BufReader::new(try!(stream.try_clone()));
    handle.set_stream(stream);
    Ok((reader))
}

#[allow(unused_must_use)]
fn reader_thread<A: ToSocketAddrs>(address: A, mut reader: BufReader<TcpStream>, event_sender: Sender<Event>, handle: Writer) {
    'read: loop {
        let mut line = String::new();
        let res = reader.read_line(&mut line);

        // If there's an error or a zero length read, we should check to reconnect or exit.
        // If the size is 0, it means that the socket was shutdown.
        if res.is_err() || res.unwrap() == 0 {
            // If the stream has the closed status, the stream was manually closed.
            if handle.is_closed() {
                // Setting stop here is irrelevant, but it removes the compiler warning.
                event_sender.send(Event::Closed);
                break;
            } else {
                // TODO: reconnection settings (delay, number of attempts)
                // The stream was not closed manually, attempt to reconnect.

                // Set the disconnected status on the writer.
                handle.disconnect();

                if event_sender.send(Event::Disconnected).is_err() {
                    break;
                }

                // Loop until reconnection is successful.
                'reconnect: loop {

                    if event_sender.send(Event::Reconnecting).is_err() {
                        break 'read;
                    }

                    match reconnect(&address, &handle) {
                        Ok(new_reader) => {
                            reader = new_reader;
                            if event_sender.send(Event::Reconnected).is_err() {
                                break 'read;
                            }

                            break 'reconnect;
                        }
                        Err(err) => {
                            if event_sender.send(Event::ReconnectionError(err)).is_err() {
                                break 'read;
                            }
                        }
                    }
                    // sleep 60 seconds
                    thread::sleep_ms(60 * 1000);
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
        handle.close();
    }
}

/// Create a connection to the given address.
///
/// A `Writer`/`Reader` pair is returned. If the connection fails,
/// an error is returned.
pub fn connect<A>(address: A) -> io::Result<(Writer, Reader)>
        // This is so I can send the address to another thread. A better solution would be nice.
        where A: ToSocketAddrs + Send + Clone + 'static {

    let stream = try!(TcpStream::connect(address.clone()));
    let reader = BufReader::new(try!(stream.try_clone()));

    let (event_sender, event_reader) = mpsc::channel::<Event>();

    let writer = Writer::new(stream);
    // The reader thread needs a handle to modify the status.
    let reader_handle = writer.clone();

    thread::spawn(move || {
        reader_thread(address, reader, event_sender, reader_handle);
    });

    Ok((writer, event_reader))
}
