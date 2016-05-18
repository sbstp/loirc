use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::io;

pub fn resolve_address<A>(addrs: A) -> io::Result<SocketAddr> where A: ToSocketAddrs {
    let sock = try!(TcpStream::connect(addrs));
    sock.peer_addr()
}
