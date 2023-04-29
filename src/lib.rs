use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread::{Builder,JoinHandle, Thread};
use std::time::Duration;

pub struct ThreadPool {
    pub(crate) workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Worker {
    id: usize,
    thread : JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}

struct Job {
    // hold the closure we want to send down channel
}

impl Worker {

    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let builder: Builder = Builder::new();
        let thread = builder.spawn(|| {
            receiver
        }).unwrap();
        return Worker { id, thread }
    }
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        return ThreadPool { workers, sender }

    }
}



