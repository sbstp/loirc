extern crate bytes;
extern crate mio;
extern crate slab;

use bytes::Buf;
use mio::{EventLoop, EventSet, Handler, PollOpt, Token, TryRead, TryWrite};
use mio::tcp::TcpStream;
use slab::Slab;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, Cursor, Read, Write};
use std::str;

struct ConnectionManager {
    lop: EventLoop<Impl>,
}

impl ConnectionManager {

    pub fn new() -> io::Result<ConnectionManager> {
        Ok(ConnectionManager {
            lop: try!(EventLoop::new()),
        })
    }

}

struct Connection {
    sock: TcpStream,
    buff: Vec<u8>,
    send_queue: VecDeque<Cursor<Vec<u8>>>,
}

fn parse(data: &[u8]) {
    println!("{:?}", str::from_utf8(data).unwrap());
}

impl Connection  {

    fn readable(&mut self) -> io::Result<()> {
        // Read until there is no more data available.
        loop {
            let size = try!(self.sock.try_read_buf(&mut self.buff));
            match size  {
                None | Some(0) => break,
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
                        if piece.ends_with(&[b'\r']) {
                            parse(&piece[0..piece.len() - 1]);
                        } else {
                            parse(piece)
                        }
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

    pub fn write(&mut self, bytes: Vec<u8>) {
        self.send_queue.push_back(Cursor::new(bytes));
    }

}

trait RawEvents {

    fn connect();

    fn disconnect();

    fn reconnect();

    fn message();

    fn error();

}

struct Impl {
    idx: usize,
    conns: HashMap<Token, Connection>,
}

impl Handler for Impl {

    type Timeout = ();
    type Message = TcpStream;

    fn ready(&mut self, lop: &mut EventLoop<Self>, token: Token, events: EventSet) {
        println!("{:?} {:?}", token, events);

        if events.is_writable() {
            self.conns.get_mut(&token).unwrap().writable();
        }
        if events.is_readable() {
            self.conns.get_mut(&token).unwrap().readable();
        }
    }

    fn notify(&mut self, lop: &mut EventLoop<Self>, sock: TcpStream) {
        let token = Token(self.idx);

        let mut conn = Connection {
            sock: sock,
            buff: Vec::with_capacity(512),
            send_queue: VecDeque::new(),
        };
        conn.write("USER simon 8 * :simon\n".as_bytes().to_vec());
        conn.write("NICK sbstp\n".as_bytes().to_vec());

        self.conns.insert(token, conn);

        lop.register(&self.conns[&token].sock, token, EventSet::readable() | EventSet::writable() | EventSet::error() | EventSet::hup(), PollOpt::edge());

        self.idx += 1;
    }

}

fn main() {
    let mut lop = EventLoop::new().unwrap();
    let chan = lop.channel();

    let addr = "54.85.60.193:6667".parse().unwrap();
    let stream = TcpStream::connect(&addr).unwrap();
    chan.send(stream);

    lop.run(&mut Impl {
        idx: 1,
        conns: HashMap::new(),
    });
}
