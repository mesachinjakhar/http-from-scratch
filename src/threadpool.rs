use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|_| {
                let receiver = Arc::clone(&receiver);
                std::thread::spawn(move || {
                    loop {
                        let job: Box<dyn FnOnce() + Send + 'static> =
                            receiver.lock().unwrap().recv().unwrap();
                        job();
                    }
                })
            })
            .collect();

        ThreadPool { workers, sender }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.send(Box::new(f)).unwrap();
    }
}
