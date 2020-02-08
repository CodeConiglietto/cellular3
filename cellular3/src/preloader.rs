use std::{
    sync::mpsc::{self, Receiver},
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
}

pub trait Generator {
    type Output: Sized;

    fn generate(&mut self) -> Self::Output;
}
