use std::sync::mpsc;
use std::thread;
use std::thread::Builder;
use std::thread::{JoinHandle, Thread};
use std::time::Duration;

pub struct ThreadPool {
     pub(crate) workers: Vec<Worker>
}

pub struct Worker {
    id: usize,
    pub(crate) thread: JoinHandle<()>
}

impl Worker {

    pub fn new(id: usize) -> Worker {
        let builder = thread::Builder::new();
        let thread = builder::spawn(|| {
            println!("I'm a worker")
        });
        return Worker { id, thread }
    }
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id))
        }
        return ThreadPool { workers }

    }
}



