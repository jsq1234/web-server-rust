use std::{sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle, Thread}};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool{
    num_threads: usize,
    workers: Vec<Worker>,
    sender: Sender<Job>
}

struct Worker{
    id: usize,
    thread: JoinHandle<()>,
}


impl Worker{
    fn new(id: usize, reciever: &Arc<Mutex<Receiver<Job>>>) -> Self{
        let recv = Arc::clone(reciever);
        let thread = thread::spawn(move || loop {
            let job = recv.lock().unwrap().recv().unwrap();
            job();
        });
        Worker { id, thread }
    }
}





impl ThreadPool {
    pub fn new(num_threads: usize) -> Self {
        assert!(num_threads > 0);
        
        let mut workers: Vec<Worker> = Vec::with_capacity(num_threads);

        let (tx, rv) = mpsc::channel::<Job>();

        let reciever = Arc::new(Mutex::new(rv));

        for i in 0..num_threads {
            let worker = Worker::new(i, &reciever);
            workers.push(worker);
        }

        ThreadPool {
            num_threads,
            workers,
            sender: tx
        }
    }

    pub fn execute<T>(&self, f: T) where T: FnOnce() + Send + 'static {
        self.sender.send(Box::new(f)).unwrap();
    }
}