use std::{
    sync::mpsc::{self, Receiver, TryRecvError},
    thread::{self, JoinHandle},
};

pub struct Preloader<T> {
    _worker_thread: JoinHandle<()>,
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
        let _worker_thread = thread::spawn(move || loop {
            if sender.send(generator.generate()).is_err() {
                break;
            }
        });

        Self {
            _worker_thread,
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

pub trait Generator {
    type Output: Sized;

    fn generate(&mut self) -> Self::Output;
}
