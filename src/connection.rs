/*!
Connection 建立客户端服务端信道

*/



use crate::socket::socket_process;

use crate::message::Message;
use crate::threads::Threads;



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
    /// 创建lspserver，创建信道，但是没有与客户端进行链接
    /// 此函数只创建connection的field，没有初始化
    pub fn lsp_server_create_tcp<T: ToSocketAddrs>(addr: T) -> std::io::Result<(Connection, Threads)>{
        // 监听 addr
        let listener = TcpListener::bind(addr)?;
        // accept 会阻塞线程直到连接
        let (stream, _) = listener.accept()?;

        println!("Heard the connection request. 已经接收到连接请求");
        println!("Generating the Channel...  正在生成信道...");

        let (connection, threads) = socket_process(stream);
        // 返回信道和信道的监听线程
        Ok((connection, threads))

    }


}


// 暂时测试不了 因为暂时没有客户端发起请求


#[test]
fn test_TCP_listener(){
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let local_addr = listener.local_addr().unwrap();
    println!("{}",listener.local_addr().unwrap());  
    
    // accept 会阻塞线程直到连接
    let (stream, _) = listener.accept().unwrap();



}