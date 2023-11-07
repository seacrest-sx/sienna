use std::thread::{self, JoinHandle};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

pub type Job = Box<dyn FnOnce() + Send + 'static>;
pub type WorkerReciever = Arc<Mutex<Receiver<Job>>>;

pub struct Worker<T = ()> {
  pub id: usize,
  pub handle: Option<JoinHandle<T>>
}

impl Worker {
    pub fn new(id: usize, receiver: WorkerReciever) -> Worker {

      let handle = thread::spawn(move || loop {
        let msg = receiver.lock().unwrap().recv();

        match msg {
            Ok(job) => {
              println!("Worker {id} got a job; executing.");
              job()
            }
            Err(_) => {
              println!("Worker {id} disconnected; shutting down.");
              break;
            }
        }
      }); 
      
      Worker { id, handle: Some(handle) }
    }
}