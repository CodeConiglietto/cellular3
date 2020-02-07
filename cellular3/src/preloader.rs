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
    pub fn new<F>(pool_size: usize, generator: F) -> Self
    where
        F: Fn() -> T,
        F: Send + 'static,
    {
        let (sender, receiver) = mpsc::sync_channel(pool_size);
        let _worker_thread = thread::spawn(move || loop {
            sender.send(generator()).unwrap();
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
