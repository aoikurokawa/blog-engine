use std::io;

use self::{executor::Executor, reactor::Reactor};

mod executor;
mod reactor;

pub fn init() -> io::Result<Executor> {
    Reactor::start()?;
    Ok(Executor::default())
}
