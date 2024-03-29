/*!
Connection 建立客户端服务端信道

*/
#[allow(warnings)]
use std::io::prelude::*;

use crate::ide_message::socket::socket_process;

use crate::ide_message::message::Message;
use crate::ide_message::threads::Threads;



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
fn test_TCP_server(){
    // bind：listener 建立在当前addr这个地址
    let listener = TcpListener::bind(("127.0.0.1", 35434)).unwrap();
    
    let local_addr = listener.local_addr().unwrap();
    println!("listener: {}",listener.local_addr().unwrap());  
    
    // accept 会阻塞线程直到连接
    let (stream, _client_socket) = listener.accept().unwrap();
    println!("client: {}",_client_socket);  

    println!("Heard the connection request. 已经接收到连接请求");
    println!("Generating the Channel...  正在生成信道...");

    let (connection, threads) = socket_process(stream);


}


// 随便写的小客户端
// 保持通讯

// 通信保持，stream 应当能够发送也能接受。两侧对等



#[test]
fn test_TCP_client()
{


    // 客户端应当有一个对等的通信条件， 同样拥有steam 以及messenger 
    let mut stream = TcpStream::connect("127.0.0.1:35434").unwrap();
    // 建立和server对等的通信?
    let buf = "first_try".to_string();
    
    stream.write(buf.as_bytes());


}
