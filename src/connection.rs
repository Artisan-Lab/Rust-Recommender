/*!
Connection 建立客户端服务端信道

*/



pub use crate::message::Message;
pub use crate::socket::socket_process;


use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use crossbeam_channel::{Receiver, Sender};

/// sender与receiver是一对通信者
/// 
/// 
pub struct Connection {
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}


impl Connection{
    /// 创建lspserver，创建信道，通过tcp连接，到此为止应当建立起一个通信信道，但是还不能使用
    /// 此函数只创建connection的field，没有初始化
    pub fn lsp_server_create_tcp<T: ToSocketAddrs>(addr: T) -> std::io::Result<Connection>{
        // 监听 addr
        // 
        let listener = TcpListener::bind(addr)?;
        // accept 会阻塞线程直到连接
        let (stream, _) = listener.accept()?;

        println!("Heard the connection request. 已经接收到连接请求");
        println!("Generating the Channel...  正在生成信道...");



    }

}