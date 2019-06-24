use super::worker::Worker;
use std::sync::{mpsc, Mutex, Arc};
use super::message::Message;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender : mpsc::Sender<Message>
}

impl ThreadPool {
    pub fn new(size : usize) -> ThreadPool{
        let mut pool = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            pool.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers: pool,
            sender
        }
    }

    pub fn execute<F>(&self, f : F)
        where F : FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for thread in &mut self.workers {
            println!("shutdown {}", thread.id);
            if let Some(thread) = thread.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}