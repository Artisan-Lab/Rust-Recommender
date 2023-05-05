/*!
 处理包装
目的是将socket加包装成 sender 和receiver

sender和receiver必须另起线程

*/

use std::net::TcpStream;
use crossbeam_channel::{bounded, Sender, Receiver};

use std::{thread, io};

use crate::ide_message::connection::Connection;
use crate::ide_message::message::Message;
use crate::ide_message::threads::Threads;


// 将给定的tcpstream 包装成sender receiver
// sender和receiver将在这里创建，但是需要在主函数结束时结束，需要把sender和receiver线程返回

// 当前先尝试向steam读或者写message




pub fn socket_process(stream: TcpStream) ->(Connection, Threads){
    
    // 抽象上，两个线程 这么视作两者关系
    //         _                                                             _ 
    //        |   client_sender       ------------------>     server_receiver |
    //  client<                                                               >server
    //        |_ client_receiver      <------------------     server_sender  _|
    //    
    // 
    // create_receiver_thread 创建第一条线程
    // create_sender_thread 创建第一条线程  这里的sender receiver是从server的角度来说的
    let (receiver, receiver_thread) = create_receiver_thread(stream.try_clone().unwrap());
    let (sender, sender_thread) = create_sender_thread(stream.try_clone().unwrap());
    
    (Connection{sender, receiver}, Threads{ receiver_thread, sender_thread})
    
}

// 注意这里message的方法都没写

fn create_receiver_thread(mut stream: TcpStream) -> (Receiver<Message>, thread::JoinHandle<io::Result<()>>){
    let (client_sender, server_recevier) = crossbeam_channel::bounded::<Message>(0);
    
    let reveiver_thread = thread::spawn(move || {
        // 消息队列
        while let Some(msg) = Message::read(&mut stream).unwrap() {
            // 
            let is_exit = msg.is_none();
            // 消息传递给receiver
            client_sender.send(msg).unwrap();
            if is_exit {
                break;
            }
        }
        Ok(())
    });
    (server_recevier, reveiver_thread)
}

fn create_sender_thread(mut stream: TcpStream) -> (Sender<Message>, thread::JoinHandle<io::Result<()>>){
    let (server_sender, client_recevier) = crossbeam_channel::bounded::<Message>(0);
    let sender_thread = thread::spawn(move || {
        client_recevier.into_iter().try_for_each(|mut it| it.write(&mut stream)).unwrap();
        Ok(())
    });
    (server_sender, sender_thread)

}
