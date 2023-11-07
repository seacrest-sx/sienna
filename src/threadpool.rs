use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

use crate::worker::Worker;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<Sender<Job>>
}


impl ThreadPool {
  pub fn spin(size: usize) -> Self {
    assert!(size > 0, "Thread pool size cannot be zero");

    let (sender, reciever) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(reciever));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender: Some(sender) }
  }
  
  pub fn execute(&self, f: impl FnOnce() + Send + 'static) {
    let job = Box::new(f);
    
    self.sender.as_ref().unwrap().send(job).unwrap();
  }
}

impl Drop for ThreadPool {
fn drop(&mut self) {
  drop(self.sender.take());

  for worker in &mut self.workers {
    println!("Shutting down worker {}", worker.id);

    if let Some(handle) = worker.handle.take() {
      handle.join().unwrap();
    }
  }
}
}