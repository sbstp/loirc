extern crate bytes;
extern crate mio;
extern crate slab;

mod code;
mod message;
mod util;

use bytes::Buf;
use mio::{EventLoop, EventSet, Handler, PollOpt, Sender, Token, TryRead, TryWrite};
use mio::tcp::TcpStream;
use slab::Slab;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, Cursor, Read, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::str;

pub use code::Code;
pub use message::Message;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ConnectionStatus {
    Closed,
    Connected,
    Disconnected,
}

pub struct Connection {
    addr: SocketAddr,
    status: ConnectionStatus,
    sock: TcpStream,
    buff: Vec<u8>,
    send_queue: VecDeque<Cursor<Vec<u8>>>,
    handler: Box<RawEvents>,
}

fn parse(data: &[u8]) -> Message {
    Message::parse(str::from_utf8(data).unwrap()).unwrap()
}

impl Connection  {

    fn status(&mut self, events: EventSet) -> bool {
        match self.status {
            ConnectionStatus::Connected if events.is_hup() || events.is_error() => {
                self.disconnect();
                return false;
            }
            ConnectionStatus::Disconnected if !(events.is_hup() || events.is_error()) => {
                self.connect();
            }
            _ => ()
        }
        true
    }

    fn perform_io(&mut self, events: EventSet) -> io::Result<()> {
        if events.is_readable() {
            try!(self.readable());
        }
        if events.is_writable() {
            try!(self.writable());
        }
        Ok(())
    }

    fn connect(&mut self) {
        self.status = ConnectionStatus::Connected;
        let w = Writer {
            send_queue: &mut self.send_queue,
        };
        self.handler.connect(w);
    }

    fn disconnect(&mut self) {
        self.status = ConnectionStatus::Disconnected;
        self.handler.disconnect();
    }

    fn readable(&mut self) -> io::Result<()> {
        // Read until there is no more data available.
        loop {
            let size = try!(self.sock.try_read_buf(&mut self.buff));
            match size  {
                None => break,
                _ => {},
            }
        }

        let mut newbuff = Vec::with_capacity(512);
        {
            let pieces: Vec<_> = self.buff.split(|b| *b == b'\n').collect();

            // If the data does not end on a line ending, it means that we read a partial message.
            // We can detect this by checking the last piece. If it isn't empty, it contains
            // a partial message.
            //
            // We must then copy this data at the start of the new buffer so that the next call to
            // try_read catches the rest of the message.
            if let Some(last) = pieces.last() {
                if !last.is_empty() {
                    newbuff.write(last);
                }
            }

            // If we have at least one message (plus the empty slice or a partial message),
            // process the messages, excluding the last one.
            if pieces.len() > 1 {
                for mut piece in pieces[0..pieces.len() - 1].iter() {
                    if !piece.is_empty() {
                        // Trim the carriage return byte if it's present.
                        let msg = if piece.ends_with(&[b'\r']) {
                            parse(&piece[0..piece.len() - 1])
                        } else {
                            parse(piece)
                        };

                        self.handler.message(Writer::new(&mut self.send_queue), msg);
                    }
                }
            }
        }

        self.buff = newbuff;
        Ok(())
    }

    fn writable(&mut self) -> io::Result<()> {
        while let Some(mut buff) = self.send_queue.pop_front() {
            let status = try!(self.sock.try_write_buf(&mut buff));

            // If the buffer was only partially written, put in back in the queue so that
            // it will be written later.
            if buff.has_remaining() {
                self.send_queue.push_front(buff);
            }

            if status.is_none() {
                break;
            }
        }
        Ok(())
    }

}

pub struct Writer<'a> {
    send_queue: &'a mut VecDeque<Cursor<Vec<u8>>>,
}

impl<'a> Writer<'a> {

    fn new<'b>(send_queue: &'b mut VecDeque<Cursor<Vec<u8>>>) -> Writer<'b> {
        Writer {
            send_queue: send_queue,
        }
    }

    pub fn write<S: Into<String>>(&mut self, cmd: S) {
        self.send_queue.push_back(Cursor::new(cmd.into().into_bytes()));
    }

}

pub trait RawEvents: Send {

    fn closed(&mut self) {}

    fn connect(&mut self, _: Writer) {}

    fn disconnect(&mut self) {}

    fn message(&mut self, _: Writer, _: Message) {}

    fn error(&mut self, _: io::Error) {}

}

struct Impl {
    conns: Slab<Connection, Token>,
}

impl Handler for Impl {

    type Timeout = ();
    type Message = Connection;

    fn ready(&mut self, lop: &mut EventLoop<Self>, token: Token, events: EventSet) {
        let conn = &mut self.conns[token];

        if conn.status(events) {
            if let Err(err) = conn.perform_io(events) {
                conn.handler.error(err);
                conn.disconnect();
            }
        }
    }

    fn notify(&mut self, lop: &mut EventLoop<Self>, conn: Connection) {
        let token = self.conns.insert(conn).ok().expect("Slab out of space.");
        lop.register(&self.conns[token].sock, token, EventSet::all(), PollOpt::edge());
    }

}

pub struct ConnectionManager {
    lop: EventLoop<Impl>,
    sender: Sender<Connection>,
}

impl ConnectionManager {

    pub fn new() -> ConnectionManager {
        let lop = EventLoop::new().unwrap();

        ConnectionManager {
            sender: lop.channel(),
            lop: lop,
        }
    }

    pub fn connect_to<A, E>(&self, addrs: A, handler: E) -> io::Result<()> where A: ToSocketAddrs, E: RawEvents + 'static {
        let addr = try!(util::resolve_address(addrs));
        let sock = try!(TcpStream::connect(&addr));

        self.sender.send(Connection {
            addr: addr,
            status: ConnectionStatus::Disconnected,
            sock: sock,
            buff: Vec::with_capacity(512),
            send_queue: VecDeque::new(),
            handler: Box::new(handler),
        });

        Ok(())
    }

    pub fn run(&mut self) {
        self.run_size(16)
    }

    pub fn run_size(&mut self, size: usize) {
        self.lop.run(&mut Impl {
            conns: Slab::new(size),
        });
    }

}
