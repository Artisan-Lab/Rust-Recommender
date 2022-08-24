/*!
 处理包装
目的是将socket加包装成 sender 和receiver

sender和receiver必须另起线程

*/

use std::net::TcpStream;
use crossbeam_channel::{bounded, Sender, Receiver};

pub use crate::connection::Connection;
use crate::connection::Message;

// 将给定的tcpstream 包装成sender receiver

pub fn socket_process(stream: TcpStream) -> Result<Connection, ()>{
    
    let (sender, recevier) = crossbeam_channel::bounded::<Message>(0);
    
    return Err(());
}




