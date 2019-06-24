use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use super::message::Message;

pub struct Worker {
    pub id : usize,
    pub thread : Option<thread::JoinHandle<()>>
}

impl Worker {
    pub fn new(id : usize, receiver : Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                match job {
                    Message::NewJob(job) => {
                        println!("{}",id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("terminate");
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread : Some(thread)
        }
    }
}
