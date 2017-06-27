use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

mod receiver;
mod sender;
mod shared;

pub use self::receiver::{Receiver, Recv};
pub use self::sender::Sender;

/// Creates a bounded, unsynchronized, multi-producer/multi-consumer channel.
///
/// It allows one or more producer tasks to dispatch to an arbitrary number of worker
/// tasks on a single Core.
///
/// Sender sinks and Receiver streams may be cloned safely. Each item sent into the
/// channel is read by only one Receiver.  Receivers are given values in the order
/// requested (i.e by polling the stream).
pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    let shared = Rc::new(RefCell::new(shared::new(capacity)));
    let rx = receiver::new(&shared);
    let tx = sender::new(shared);
    (tx, rx)
}


#[derive(Debug)]
pub struct SenderLost();
impl fmt::Display for SenderLost {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "recv failed because sender is gone")
    }
}
impl Error for SenderLost {
    fn description(&self) -> &str {
        "recv failed because sender is gone"
    }
}