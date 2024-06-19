use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;
type Error = Box<dyn std::error::Error>;

enum Message {
    NewJob(Job),
    Terminate
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers  = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers: workers, sender: sender }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Error>  where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job))?;

        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate);
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
            .lock()
            .unwrap()
            .recv()
            .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} received a new job, executing", id);
                    job();
                },
                Message::Terminate => {
                    println!("Terminating job in Worker {}", id);
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}
