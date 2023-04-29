use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread::{Builder,JoinHandle, Thread};
use std::time::Duration;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Worker {
    id: usize,
    thread : JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl Worker {

    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let builder: Builder = Builder::new().name("my-thread".into()).stack_size(1024 * 1024);
        let thread = builder.spawn(move || loop {
            let result = receiver.lock().unwrap().recv();
            match result {
                Ok(job) => {
                    println!("Worker {id} Successfully handle job");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected, shutting down");
                    break;
                }
            }

        }).unwrap();
        return Worker { id, thread }
    }

}

impl ThreadPool {

// pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        return ThreadPool { workers, sender }

    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}



