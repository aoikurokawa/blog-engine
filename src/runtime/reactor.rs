use std::{
    collections::HashMap,
    io,
    task::{Context, Waker},
};

use mio::{net::TcpStream, Events, Interest, Poll, Registry, Token};

pub struct Reactor {
    wakers: HashMap<usize, Waker>,
    registry: Registry,
    next_id: usize,
}

impl Reactor {
    pub fn new(wakers: HashMap<usize, Waker>, registry: Registry) -> Self {
        Self {
            wakers,
            registry,
            next_id: 1,
        }
    }

    pub fn register(
        &self,
        source: &mut TcpStream,
        id: usize,
        interests: Interest,
    ) -> io::Result<()> {
        self.registry.register(source, Token(id), interests)
    }

    pub fn set_waker(&mut self, id: usize, cx: &Context) {
        self.wakers.insert(id, cx.waker().clone());
    }

    pub fn deregister(&mut self, source: &mut TcpStream, id: usize) -> io::Result<()> {
        self.wakers.remove(&id);
        self.registry.deregister(source)
    }

    pub fn next_id(&self) -> usize {
        self.next_id + 1
    }

    fn event_loop(&self, mut poll: Poll) -> io::Result<()> {
        let mut events = Events::with_capacity(100);
        loop {
            poll.poll(&mut events, None)?;
            for e in events.iter() {
                let Token(id) = e.token();

                if let Some(waker) = self.wakers.get(&id) {
                    waker.wake_by_ref();
                }
            }
        }
    }

    pub fn start() -> io::Result<()> {
        let wakers = HashMap::new();
        let poll = Poll::new()?;
        let registry = poll.registry().try_clone()?;
        let reactor = Reactor::new(wakers, registry);

        std::thread::spawn(move || reactor.event_loop(poll));

        Ok(())
    }
}
