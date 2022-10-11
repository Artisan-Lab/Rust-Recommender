use std::fs::File;
use std::path::Path;
use std::io::{self,prelude::*};

// 枚举当前变量的类型，不在乎类型本身
// 是否存在问题？
pub enum Var_type{
    Owner,// 所有权
    Mutref,// 可变引用
    Stref,// 不可变引用

}



// 变量 基本单位 
pub struct Var{
    // string 变量的名称
    s: String,
    // 关注变量的类型
    t: Var_type,

}
// 测试读入文件 把文件转换成字符串存入
#[test]
fn reader() 
{
    // 当前假设所有出现错误的var 名字都叫va
    //let path = "tester.rs";
    let file = File::open(Path::new("./src/parse/tester.rs"))
        .expect("Open file failed");
    // 读入成功
    // lines 是一个迭代器 从第0行开始，next是第一行
    let mut line_iter = io::BufReader::new(file).lines();
    // 第一行 
    let mut lines = line_iter.next()
        .expect("Read file failed");
    println!("{:?}",lines);
    while true{
        if lines.unwrap().is_empty(){
            break;
        }
        lines = line_iter.next()
            .expect("reading wrong");
        println!("{:?}",lines);
    }

}

