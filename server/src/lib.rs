use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use sysinfo::System;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool. 
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); 
        }

        ThreadPool { 
            workers, 
            sender: Some(sender),
        }
    }

    /// Executes a closure that one of the workers will pick up. 
    ///
    /// # Panics
    ///
    /// The `execute` function panics when the sender reference or job does not exist.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Drops workers for shutting down threadpool.
    ///
    /// # Panics
    ///
    /// The `drop` fn panics when worker.thread.join() is an Err().
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        
        Worker { id, thread }
    }
}

pub struct Hardware {
    pub logical_processors: usize,
    pub os: String,
}

impl Hardware {
    pub fn new() -> Hardware {
        let sys = System::new_all();
        let logical_processors = sys.cpus().len();
        let os = System::long_os_version().unwrap_or(String::from("Operating system not found!"));
        println!("Logical processors found: {}, OS found: {}", logical_processors, os); 
        Hardware { logical_processors, os: os }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn test_new_hardware_logical_processors() {
        let hardware = Hardware::new();
        assert!(hardware.logical_processors > 0);
    }

    #[test]
    fn test_new_hardware_os_found() {
        let hardware = Hardware::new();
        assert!(hardware.os.len() > 0);
    }

}
