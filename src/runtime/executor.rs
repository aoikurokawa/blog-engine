use std::{
    collections::HashMap,
    future::Future,
    io,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
    thread::{self, Thread},
};

type Task = Pin<Box<dyn Future<Output = io::Result<()>>>>;

#[derive(Default)]
pub struct Executor {
    tasks: HashMap<usize, Task>,
    ready_queue: Vec<usize>,
    next_id: usize,
}

impl Executor {
    fn pop_ready(&mut self) -> Option<usize> {
        self.ready_queue.pop()
    }

    fn get_future(&mut self, id: usize) -> Option<Task> {
        self.tasks.remove(&id)
    }

    fn get_waker(&self, id: usize) -> Arc<MyWaker> {
        Arc::new(MyWaker {
            thread: thread::current(),
            ready_queue: Arc::new(Mutex::new(self.ready_queue.clone())),
            id,
        })
    }

    fn insert_task(&mut self, id: usize, task: Task) {
        self.tasks.insert(id, task);
    }

    fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn block_on<F>(&mut self, future: F)
    where
        F: Future<Output = io::Result<()>> + 'static,
    {
        let id = self.next_id;
        self.tasks.insert(id, Box::pin(future));
        self.ready_queue.push(id);
        self.next_id += 1;

        loop {
            while let Some(id) = self.pop_ready() {
                let mut future = match self.get_future(id) {
                    Some(f) => f,
                    None => continue,
                };

                let waker: Waker = self.get_waker(id).into();
                let mut cx = Context::from_waker(&waker);

                match future.as_mut().poll(&mut cx) {
                    Poll::Ready(_) => continue,
                    Poll::Pending => self.insert_task(id, Box::pin(future)),
                }
            }

            let task_count = self.task_count();
            let name = thread::current().name().unwrap_or_default().to_string();

            if task_count > 0 {
                println!("{name}: {task_count} pending tasks. Sleep until notified.");
                thread::park();
            } else {
                println!("{name}: All tasks are finished");
                break;
            }
        }
    }
}

#[derive(Clone)]
pub struct MyWaker {
    thread: Thread,
    id: usize,
    ready_queue: Arc<Mutex<Vec<usize>>>,
}

impl Wake for MyWaker {
    fn wake(self: Arc<Self>) {
        self.ready_queue
            .lock()
            .map(|mut q| q.push(self.id))
            .unwrap();
        self.thread.unpark();
    }
}
