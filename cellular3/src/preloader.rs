use std::{
    sync::{
        mpsc::{self, Receiver, TryRecvError},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct Preloader<T>
where
    T: Send + 'static,
{
    worker_thread: Option<JoinHandle<()>>,
    running: Arc<Mutex<bool>>,
    receiver: Receiver<T>,
}

impl<T> Preloader<T>
where
    T: Send + 'static,
{
    pub fn new<G>(pool_size: usize, mut generator: G) -> Self
    where
        G: Generator<Output = T> + Send + 'static,
    {
        let (sender, receiver) = mpsc::sync_channel(pool_size);
        let running = Arc::new(Mutex::new(true));
        let running_worker = Arc::clone(&running);

        let worker_thread = Some(thread::spawn(move || loop {
            if sender.send(generator.generate()).is_err() || !*running_worker.lock().unwrap() {
                break;
            }
        }));

        Self {
            worker_thread,
            running,
            receiver,
        }
    }

    pub fn get_next(&self) -> T {
        self.receiver.recv().unwrap()
    }

    pub fn try_get_next(&self) -> Option<T> {
        match self.receiver.try_recv() {
            Ok(item) => Some(item),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Worker thread disconnected"),
        }
    }
}

impl<T> Drop for Preloader<T>
where
    T: Send + 'static,
{
    fn drop(&mut self) {
        println!("Shutting down preloader thread");
        *self.running.lock().unwrap() = false;
        self.worker_thread.take().unwrap().join().unwrap();
    }
}

pub trait Generator {
    type Output: Sized;

    fn generate(&mut self) -> Self::Output;
}
