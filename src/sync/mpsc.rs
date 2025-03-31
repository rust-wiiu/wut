use super::mpmc;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = mpmc::channel();
    (Sender { inner: tx }, Receiver { inner: rx })
}

#[derive(Clone)]
pub struct Sender<T> {
    inner: mpmc::Sender<T>,
}

impl<T> Sender<T> {
    pub fn send(&self, data: T) -> Result<(), ()> {
        self.inner.send(data)
    }
}

unsafe impl<T> Sync for Sender<T> {}
unsafe impl<T: Send> Send for Sender<T> {}

pub struct Receiver<T> {
    inner: mpmc::Receiver<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Result<T, ()> {
        self.inner.recv()
    }
}

unsafe impl<T> Sync for Receiver<T> {}
unsafe impl<T: Send> Send for Receiver<T> {}
