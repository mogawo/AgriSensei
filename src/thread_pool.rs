pub use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool 
    {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size
        {
            workers.push(Worker::new(id, receiver.clone()));
        }
        ThreadPool {workers, sender: Some(sender)}
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job); //TODO Handle std::sync::mpsc::SendError
    }
}




struct Worker
{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

struct Avg{
    pub sum: f64,
    pub count: f64,
    pub avg: f64
}

impl Avg{
    pub fn new() -> Self{
        Avg{
            sum: 0.,
            count: 1.,
            avg: 0.
        }
    }
    pub fn add(&mut self, x: f64){
        self.sum += x;
        self.count += 1.;
        self.avg = self.sum / self.count;
    }
}

use std::time::{Instant, Duration};
impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let mut avg = Avg::new();
        let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();
                match message{
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        let now = Instant::now();
                        job();
                        let elap = now.elapsed();
                        println!("Took {:#?}", elap);
                        avg.add(elap.as_secs() as f64);
                        // println!("Average Process time = {}, count = {}", avg.avg, avg.count);
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            });
        Worker{id, thread: Some(thread)}
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

