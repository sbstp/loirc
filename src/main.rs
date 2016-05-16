extern crate mio;
extern crate slab;

use mio::{EventLoop, EventSet, Handler, PollOpt, Token, TryRead};
use mio::tcp::TcpStream;
use std::collections::HashMap;
use std::collections::VecDeque;
use slab::Slab;
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
    send: bool,
}

impl Connection  {

    fn readable(&mut self) -> io::Result<()> {
        loop {
            let size = try!(self.sock.try_read_buf(&mut self.buff));
            //println!("{:?}", size);
            match size  {
                None => break,
                Some(0) => break,
                _ => {},
            }
        }

        let mut newbuff = Vec::new();
        {
            let pieces: Vec<_> = self.buff.split(|b| *b == b'\n').collect();

            if let Some(last) = pieces.last() {
                if last.len() > 0 {
                    newbuff.write(last);
                }
            }

            // todo out of bounds
            for piece in pieces[0..pieces.len() - 1].iter() {
                println!("{:?}", str::from_utf8(piece).unwrap().trim_right());
            }
        }

        self.buff = newbuff;
        Ok(())
    }

    fn writable(&mut self) {
        if self.send {
            write!(self.sock, "USER simon 8 * :simon\n");
            write!(self.sock, "NICK sbstp\n");
            self.send = false;
        }
    }

}

struct Impl {
    idx: usize,
    conns: HashMap<Token, Connection>,
}

impl Handler for Impl {

    type Timeout = ();
    type Message = TcpStream;

    fn ready(&mut self, lop: &mut EventLoop<Self>, token: Token, events: EventSet) {
        //println!("{:?} {:?}", token, events);

        if events.is_writable() {
            self.conns.get_mut(&token).unwrap().writable();
        }
        if events.is_readable() {
            self.conns.get_mut(&token).unwrap().readable();
        }
    }

    fn notify(&mut self, lop: &mut EventLoop<Self>, sock: TcpStream) {
        let token = Token(self.idx);

        let conn = Connection {
            sock: sock,
            buff: Vec::new(),
            send: true,
        };

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
