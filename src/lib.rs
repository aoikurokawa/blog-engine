use std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
};

use runtime::reactor::Reactor;

pub mod handlers;
pub mod runtime;
pub mod startup;

pub struct Async {
    listener: TcpListener,
}

impl Async {
    pub fn new(listener: TcpListener) -> io::Result<Self> {
        listener.set_nonblocking(true)?;
        Ok(Self { listener })
    }

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
match TcpListener::bind(addr) {
            Ok(listner) => 
        Async::new(listner),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {

            }
            Err(e) => Err(e)
        }
    }

    pub async fn accept(&self) -> io::Result<TcpStream> {
        loop {
            match self.listener.accept() {
                Ok((stream, _)) => {
                    return Ok(stream);
                }
                Err(e) => eprintln!("{e}"),
            }
        }
    }
}
