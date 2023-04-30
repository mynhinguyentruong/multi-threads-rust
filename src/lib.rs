use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::{Builder, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let builder = Builder::new().name("builder".into());

        let thread = builder.spawn(move || {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        }).unwrap();

        return Worker { id, thread: Some(thread) }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            // drop
            println!("Shutting down worker {}", worker.id);
            if let Some(some_weird_shit) = worker.thread.take() {
                some_weird_shit.join().unwrap();
            }
        }
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

    pub fn execute<A>(&self, f: A)
    where
        A: FnOnce() + Send + 'static
    {
        let job: Job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
