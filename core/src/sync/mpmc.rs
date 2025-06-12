use alloc::{boxed::Box, sync::Arc};
use core::marker::PhantomData;
use wut_sys as sys;

const QUEUE_LENGTH: usize = 32;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let queue = Arc::new(sys::OSMessageQueue::default());
    let messages = Arc::new([sys::OSMessage::default(); QUEUE_LENGTH]);

    unsafe {
        // this is very naughty
        sys::OSInitMessageQueue(
            Arc::as_ptr(&queue) as *mut _,
            Arc::as_ptr(&messages) as *mut _,
            messages.len() as i32,
        );
    };

    (
        Sender::new(&queue, &messages),
        Receiver::new(&queue, &messages),
    )
}

#[derive(Clone)]
pub struct Sender<T> {
    queue: Arc<sys::OSMessageQueue>,
    _messages: Arc<[sys::OSMessage; QUEUE_LENGTH]>,
    _marker: PhantomData<T>,
}

impl<T> Sender<T> {
    fn new(
        queue: &Arc<sys::OSMessageQueue>,
        messages: &Arc<[sys::OSMessage; QUEUE_LENGTH]>,
    ) -> Self {
        Self {
            queue: Arc::clone(queue),
            _messages: Arc::clone(messages),
            _marker: PhantomData,
        }
    }

    fn get_queue(&self) -> *mut sys::OSMessageQueue {
        Arc::as_ptr(&self.queue) as *mut _
    }

    pub fn send(&self, data: T) -> Result<(), ()> {
        let data = Box::new(Box::new(data));
        let mut msg = sys::OSMessage::default();
        msg.message = Box::into_raw(data) as *mut core::ffi::c_void;

        let success = unsafe {
            sys::OSSendMessage(
                self.get_queue(),
                &mut msg,
                sys::OSMessageFlags::OS_MESSAGE_FLAGS_BLOCKING,
            )
        };

        if success == sys::FALSE as _ {
            Err(())
        } else {
            Ok(())
        }
    }
}

unsafe impl<T> Sync for Sender<T> {}
unsafe impl<T: Send> Send for Sender<T> {}

#[derive(Clone)]
pub struct Receiver<T> {
    queue: Arc<sys::OSMessageQueue>,
    _messages: Arc<[sys::OSMessage; QUEUE_LENGTH]>,
    _marker: PhantomData<T>,
}

impl<T> Receiver<T> {
    fn new(
        queue: &Arc<sys::OSMessageQueue>,
        messages: &Arc<[sys::OSMessage; QUEUE_LENGTH]>,
    ) -> Self {
        Self {
            queue: Arc::clone(queue),
            _messages: Arc::clone(messages),
            _marker: PhantomData,
        }
    }

    fn get_queue(&self) -> *mut sys::OSMessageQueue {
        Arc::as_ptr(&self.queue) as *mut _
    }

    pub fn recv(&self) -> Result<T, ()> {
        let mut msg = sys::OSMessage::default();

        let success = unsafe {
            sys::OSReceiveMessage(
                self.get_queue(),
                &mut msg,
                sys::OSMessageFlags::OS_MESSAGE_FLAGS_BLOCKING,
            )
        };

        if success == sys::FALSE as _ {
            Err(())
        } else {
            Ok(unsafe { **Box::from_raw(msg.message as *mut Box<T>) })
        }
    }
}

unsafe impl<T> Sync for Receiver<T> {}
unsafe impl<T: Send> Send for Receiver<T> {}
