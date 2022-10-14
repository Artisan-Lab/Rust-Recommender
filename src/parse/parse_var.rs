use std::fs::File;
use std::path::Path;
use std::io::{self,prelude::*};
use std::ptr::read;
use std::vec;

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

// 只在函数内部的cfg 要考虑函数间调用，但是视作一个图中的普通节点
// 这是对单个函数做的分析，先不考虑其他函数
pub enum node_type{
    ENTRY,
    MIXED,
    USED,
    DUMMY,
    SEQUENTIAL,
    FN,
}

#[derive(Clone)]
pub struct node{
    // fn_type: node_type,

}

impl node {
    pub fn new() -> node{
        node {  }
    }
}




fn reader() -> Vec<String> 
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
        .expect("Read file failed")
        .expect("Read file failed");
    // println!("{:?}",lines);
    // vec lines 保存了整个文档的所有代码到一个vec里面，
    // 暂时没有搞多个文档
    let mut vec_lines: Vec<String> = Vec::new(); 
    vec_lines.push(lines);
    while true{
        
        
        lines = line_iter.next()
            .expect("reading wrong")
            .expect("character wrong");
        // println!("{:?}",lines);
        // 文档以 // end为结尾
        if lines == "// end"{
            break;
        }
        vec_lines.push(lines);
    }
    //
    // println!("{:?}", vec_lines);
    vec_lines
}

// 获取到了文档的每一行后进行处理

// 1 找到所有va的别名 包括(函数签名 + 函数返回值 + 变量内部方法) 
// 2 别名在同一个域的出现顺序 以及所在域 -> 识别 {} 和函数 ()
// 3 给别名做域的区分，包括函数传参和函数参数 函数返回值
//  先最简单版 不判断任何类型? (类型很重要，需要区分copy等等)
//  这个可以先适合 e502?

//
fn deal_vec_lines(vec_lines: Vec<String>){
    // 第一步 ， 先遍历找到va
    // 分离单行 找到变量
    
    let mut a = 1;
    // va 出现的行数
    let mut va_line_number = 0;

    // 提取词汇
    for s in vec_lines.iter(){
        let fields: Vec<&str> = s //  || c == ','?
            .split(|c| c == ' '  || c == '{' || c == '}')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        // println!("{:?}",fields);
        // ***遍历每一行的 fileds 先找到变量va 假设找到va的第一次出现
        for s2 in fields{
            if s2 == "//" || s2 == "/*" {
                break;
            }
            if s2 == "va" {
                va_line_number = a;
                println!("found va");
                break;
            }
            
        }
        a = a + 1;
    
    }
    // 假设这里va是报错位置，已经获得了va的报错位置
    // 需要寻找va的定义
    println!("{}",va_line_number);

}


#[test]
fn deal_test() {
    let v = reader();
    // 获取到了每一行的内容 a是当前的数字
    let a= deal_vec_lines(v);
}

#[test]
fn callee_test() {
    

}



// 测试读入文件 把文件转换成字符串存入
#[test]
fn reader_test() 
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
        .expect("Read file failed")
        .expect("Read file failed");
    // println!("{:?}",lines);
    // vec lines 保存了整个文档的所有代码到一个vec里面，
    // 暂时没有搞多个文档
    let mut vec_lines: Vec<String> = Vec::new(); 
    vec_lines.push(lines);
    while true{
        
        
        lines = line_iter.next()
            .expect("reading wrong")
            .expect("character wrong");
        // println!("{:?}",lines);
        // 文档以 // end为结尾
        if lines == "// end"{
            break;
        }
        vec_lines.push(lines);
    }
    //
    println!("{:?}", vec_lines);

}




