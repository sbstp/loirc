use bytes::Buf;
use mio::{EventLoop, EventSet, Handler, PollOpt, Sender, Token, TryRead, TryWrite};
use mio::tcp::TcpStream;
use slab::Slab;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, Cursor, Read, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::str;
use std::time::Duration;

use code::Code;
use message::Message;
use util;
use ::{MonitorSettings, RawEvents};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectionStatus {
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
    rec_delay: Option<Duration>,
    mon_set: Option<MonitorSettings>,
}

fn parse(data: &[u8]) -> Message {
    Message::parse(str::from_utf8(data).unwrap()).unwrap()
}

impl Connection  {

    fn perform_io(&mut self, events: EventSet) -> io::Result<()> {
        if events.is_readable() {
            try!(self.readable());
        }
        if events.is_writable() {
            try!(self.writable());
        }
        Ok(())
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

enum Directive {
    Monitor,
    Reconnect(Token),
}

struct Impl {
    conns: Slab<Connection, Token>,
}

impl Handler for Impl {

    type Timeout = Directive;
    type Message = Connection;

    fn ready(&mut self, lop: &mut EventLoop<Self>, token: Token, events: EventSet) {
        println!("{:?}", events);
        let conn = &mut self.conns[token];

        if conn.status == ConnectionStatus::Disconnected && (events.is_readable() || events.is_writable()) {
            conn.status = ConnectionStatus::Connected;
            conn.handler.connect(Writer::new(&mut conn.send_queue));
        }

        // Try to perform IO. If it fails, trigger the error event.
        if events.is_hup() || events.is_error() || conn.perform_io(events).is_err() {
            conn.status = ConnectionStatus::Disconnected;
            conn.handler.disconnect();

            if let Some(delay) = conn.rec_delay {
                lop.timeout_ms(Directive::Reconnect(token), delay.as_secs() * 1000);
            }
        }
    }

    fn timeout(&mut self, lop: &mut EventLoop<Impl>, dir: Directive) {
        match dir {
            Directive::Monitor => {
                for conn in self.conns.iter_mut() {
                }
                lop.timeout_ms(Directive::Monitor, 1000);
            }
            Directive::Reconnect(token) => {
                let conn = &mut self.conns[token];
                if conn.status == ConnectionStatus::Disconnected {
                    if let Ok(sock) = TcpStream::connect(&conn.addr) {
                        lop.register(&sock, token, EventSet::all(), PollOpt::edge());
                        conn.sock = sock;
                    }
                }
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

    pub fn connect<A, E>(&self, addrs: A, handler: E, rec_delay: Option<Duration>, mon_set: Option<MonitorSettings>)
        -> io::Result<()> where A: ToSocketAddrs, E: RawEvents + 'static
    {
        let addr = try!(util::resolve_address(addrs));
        //let addr = "168.235.69.209:6667".parse().unwrap();
        let sock = try!(TcpStream::connect(&addr));

        self.sender.send(Connection {
            addr: addr,
            status: ConnectionStatus::Disconnected,
            sock: sock,
            buff: Vec::with_capacity(512),
            send_queue: VecDeque::new(),
            handler: Box::new(handler),
            rec_delay: rec_delay,
            mon_set: mon_set,
        });

        Ok(())
    }

    pub fn run(&mut self) {
        self.run_size(16)
    }

    pub fn run_size(&mut self, size: usize) {
        self.lop.timeout_ms(Directive::Monitor, 1000);
        self.lop.run(&mut Impl {
            conns: Slab::new(size),
        });
    }

}
