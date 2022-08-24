/*!
Connection 建立客户端服务端信道

*/

mod message;
pub use crate::message::Message;

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
        if let Ok((stream, _)) = listener.accept(){
            println!("Heard the connection request.");
            println!("Generating the Channel...");
            stream

        } else{
            return Err();
        }
        
    }

}