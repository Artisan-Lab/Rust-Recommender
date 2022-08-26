use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Read};



/// Message 信道消息传递的基本单位，需要合适设计
/// 先试一个string，不在乎serdejson
/// serde_json
pub struct Message{
    text: String,
}

impl Message{
    // message 将信息读取到sender
    pub fn read(stream: &mut TcpStream) -> Result<Option<Message>,()>{
        // 解tcpstream成信息
        // 创建一个bufreader实例?
        // println!("1231231231");
        let mut reader = BufReader::new(stream);
        let mut buf = String::new();
        // reader.read_line(&mut buf);

        println!("server stream read:{}",buf);

            

        Err(())
    }
    // message 将信息传输到receiver
    pub fn write(&mut self, stream: &mut TcpStream) -> Result<(),()>{
        Err(())
    }
    pub fn is_none(&self) -> bool{
        return false;
    }
}