
use std::{thread,io};


pub struct Threads {
    pub receiver_thread: thread::JoinHandle<io::Result<()>>,
    pub sender_thread: thread::JoinHandle<io::Result<()>>,
}
// 信道结束
impl Threads {
    pub fn join(self) {
        self.receiver_thread.join().expect("receiver_thread panicked!");
        self.sender_thread.join().expect("sender_thread panicked!");
    }
}