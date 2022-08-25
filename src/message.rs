use std::net::TcpStream;
use std::io;



/// Message 信道消息传递的基本单位
/// serde_json
pub struct Message{
    
}

impl Message{
    // message 将信息读取到sender
    pub fn read(stream: &mut TcpStream) -> Result<Option<Message>,()>{
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