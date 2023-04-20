

use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::path::Path;
use std::io::{self,prelude::*};
use std::ptr::read;
use std::{vec, string};


use csv;
use proc_macro2::Ident;

use super::adjlist::Adjlist;
use super::alias_analysis;
use super::method_call::method_call_names;

use log::{debug, info};

use syn::{self, Stmt, Local, Pat, PatIdent, Expr, Signature, ExprReference};



// 11.17 bug 为什么max的定义语句会出现两次啊。。。

// 枚举当前变量的类型，不在乎类型本身
// 是否存在问题？
// 变量 基本单位 

#[derive(Clone,Debug)]
pub struct VarInfo{
    // string 变量的名称
    pub Name: Option<String>,
    pub Reference: bool,
    pub Mutability: bool,
    pub Number: usize, // Number 表示在图中的编号
    // 关注变量的类型
    // todo::变量类型 结构体
}

// 函数语句，考虑名字以外还有变量？ 参数/返回值？
#[derive(Clone,Debug)]

// function call 与 methodcall合并
pub struct FuncInfo {
    pub Name: Option<String>,
    pub return_value: usize, //  返回值用什么表示？    0:没有返回值/返回值非引用 1:返回值& 2:&mut
    pub Number:usize,
    pub Start: bool,
    pub End: bool,
    pub method_call: i32,
    // method_call -1表示不是方法调用 0表示self 1表示&self 2表示 mut self 3表示 &mut self
}
// 新增返回值 传参数量变成返回值
// 对signature需要解path tuple typereference

// 只在函数内部的cfg 要考虑函数间调用，但是视作一个图中的普通节点
// 这是对单个函数做的分析，先不考虑其他函数

// node
#[derive(Clone,Debug)]
pub enum stmt_node_type {
    Owner(VarInfo),
    MutRef(VarInfo),
    StaticRef(VarInfo),
    Function(FuncInfo),
}

#[derive(Clone,Debug)]
pub enum block_node_type{
    BLOCK_START,
    BLOCK_END,
    BLOCK_NONE,
}



// 图中的一个节点应当是block类型或者是普通语句类型
// block 代表括号引起的scope分割
// stmt 代表一个变量的使用/声明等等
#[derive(Clone,Debug)]
// Todo ：return 节点
pub struct node{

    pub stmt: Option<stmt_node_type>,
    pub block: Option<block_node_type>,
}

// block节点插入

// graph.push(node { 
//     stmt: None, 
//     block: Some(block_node_type::BLOCK_START) 
// });
// graph.add(graph.len_num()-2, graph.len_num()-1);


impl node {
    pub fn new(stmt:Option<stmt_node_type>, block : Option<block_node_type>) -> node{
        node { stmt, block }
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
        // 每一行的fields 代表去掉空格之后的一个单词
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
    // println!("{}",va_line_number);

}

#[test]
fn deal_test() {
    let v = reader();
    // 获取到了每一行的内容 a是当前的数字
    let a= deal_vec_lines(v);
}

// 处理一个hash表 获取是所有method name 以及返回值？  返回值意义?


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
    // println!("{:?}", vec_lines);

}





// 解析syn 生成图 提取attribute? // 假设已经知道 所需变量？
fn graph_generate(ast: &syn::File, funcname: String, var_set: &mut HashMap<String,(i32,bool,bool)>/*  别名表*/, method_map: &HashMap<String,(i32,usize)>/*methodinfo */, askfunction: &str) -> Adjlist  {
    // 遍历item

    let mut graph = Adjlist::new();

    for item in &ast.items{
        // match fn 对于fn对象来构成图 // 寻找函数名称
        match item{
            syn::Item::Fn(func) => {
                // println!("{:?}",func.sig.ident);

                // for arg in &func.sig.inputs {} 
                // 对fn先创建fn节点
                
                // 当前函数入口认为是0节点

                

                if func.sig.ident == askfunction.to_string(){
                    graph.push(node { stmt:Some(stmt_node_type::Function(FuncInfo{Name: Some(askfunction.to_string()), return_value: 0 , Number: graph.len_num(), Start:true, End:false, method_call: -1})) , block: None });
                    // todo? 需要传入前后节点吗?
                    // 用节点标号构图 节点标号不能有错误

                    // signature
                    for arg in &func.sig.inputs {
                        match arg {
                            syn::FnArg::Typed(pattyped) => {
                                // patident 是名字
                                match &*pattyped.pat{
                                    Pat::Ident(patident) => {
                                        if var_set.contains_key(&String::from(format!("{}",patident.ident))){
                                            
                                            let var_str = String::from(format!("{}",patident.ident));
                                            if let Some(value) = var_set.get(&var_str) {
                                                graph.push_node(value, &var_str);
                                                graph.add(graph.len_num()-2, graph.len_num()-1);
                                            }
                                            else { println!("wrong value of hashmap");}
                            
                                        }
                                    }
                                    _ => {}
                                }
                                
                                
                            }
                            _ => {}
                        }
                    } 

                    for stmt in &func.block.stmts {
                        // 传入图 别名表
                        let graph_num = graph.len_num();
                        graph_stmt(&stmt , var_set, &mut graph, method_map , graph_num-1);

                    }
                    
                }
                
                

            }
            // 函数在impl块内
            syn::Item::Impl(Itemimpl) => {
                for method in &Itemimpl.items{
                    match method {
                        syn::ImplItem::Method(itemMethod) => {
                            if itemMethod.sig.ident == askfunction.to_string() {
                                // 如果是方法的
                                graph.push(node { stmt:Some(stmt_node_type::Function(FuncInfo{Name: Some(askfunction.to_string()), return_value: 0 , Number: graph.len_num(), Start:true, End:false, method_call: -1})) , block: None });
                                for arg in &itemMethod.sig.inputs {
                                    match arg {
                                        syn::FnArg::Typed(pattyped) => {
                                            // patident 是名字
                                            match &*pattyped.pat{
                                                Pat::Ident(patident) => {
                                                    if var_set.contains_key(&String::from(format!("{}",patident.ident))){
                                                        
                                                        let var_str = String::from(format!("{}",patident.ident));
                                                        if let Some(value) = var_set.get(&var_str) {
                                                            graph.push_node(value, &var_str);
                                                            graph.add(graph.len_num()-2, graph.len_num()-1);
                                                        }
                                                        else { println!("wrong value of hashmap");}
                                        
                                                    }
                                                }
                                                _ => {}
                                            } 
                                        }
                                        _ => {}
                                    }
                                } 

                                // signature

                                for stmt in &itemMethod.block.stmts {
                                    let graph_num = graph.len_num();
                                    graph_stmt(&stmt , var_set, &mut graph, method_map , graph_num-1);
                                }
                            }
                        }
                        
                        _ => ()
                    }
                    
                }
            }
            _ => () // 
        }
    }   
    // test
    // graph.show();

    // println!("{:#?}",graph);
    graph

}

// 解析节点插入

fn graph_block (block: &syn::Block, var_def: &mut HashMap<String,(i32,bool,bool)>,graph: &mut Adjlist, method_map: &HashMap<String,(i32,usize)> ,last_node_num: usize){
    // 建图需要 前后加block节点

    // Todo： stmt中可能没有任何hashset变量出现，可能不需要插入任何节点 是否需要stmt中不进行任何节点插入操作，
    // 暂且不考虑不出现的问题，等待图构建完成后再进行搜索修图？
    graph.push_block_start();
    graph.add(last_node_num, graph.len_num()-1);
    //
    for stmt in &block.stmts {
        graph_stmt(stmt, var_def, graph, method_map,graph.len_num()-1); 
        // 存在可能性stmt中没有任何一个节点需要添加？
    }
    //
    graph.push_block_end();
    graph.add(graph.len_num()-2, graph.len_num()-1);
}
// 普通的pat
fn graph_pat (pat: &syn::Pat, var_def: &mut HashMap<String,(i32,bool,bool)>,graph: &mut Adjlist, last_node_num: usize)
{
    match pat {
        Pat::Ident(patident) => {
            if var_def.contains_key(&String::from(format!("{}",patident.ident))){
                
                let var_str = String::from(format!("{}",patident.ident));
                if let Some(value) = var_def.get(&var_str) {
                    graph.push_node(value, &var_str);
                    graph.add(graph.len_num()-2, graph.len_num()-1);
                }
                else { println!("wrong value of hashmap");}

            }
        }
        Pat::Tuple(pattuple) => {
            for element in &pattuple.elems {
                graph_pat(element, var_def, graph, graph.len_num()-1);
            }
        }
        _=> {}
    }
}
// pat2 一个新的临时变量，一定是& / &mut，必须设置成引用
fn graph_pat2 (pat: &syn::Pat, var_def: &mut HashMap<String,(i32,bool,bool)>,graph: &mut Adjlist, last_node_num: usize, new_mut:bool)
{
    match pat {
        Pat::Ident(patident) => {
            if var_def.contains_key(&String::from(format!("{}",patident.ident))){
                
                let var_str = String::from(format!("{}",patident.ident));
                if let Some(value) = var_def.get(&var_str) {
                    let mut new_value = (-1,false,false);
                    if new_mut{
                        new_value = (2,true,true);
                    }else{
                        new_value = (3,true,false);
                    }
                    graph.push_node(&new_value, &var_str);
                    graph.add(graph.len_num()-2, graph.len_num()-1);
                }
                else { println!("wrong value of hashmap");}

            }
        }
        Pat::Tuple(pattuple) => {
            for element in &pattuple.elems {
                graph_pat2(element, var_def, graph, graph.len_num()-1,new_mut);
            }
        }
        _=> {}
    }
}



// func名称解析
fn path_fmt(exprpath : &syn::ExprPath) -> String {
    let mut pathname = "".to_owned();
    for seg in exprpath.path.segments.iter() {
        pathname.push_str(&seg.ident.to_string());
        pathname.push_str(&String::from("::"));
        // println!("{:?}", seg);
    }
    pathname[0..pathname.len()-2].to_string()
}

// 对一个表达式进行构图，关键是寻找出现的变量是否在hashset之中\
fn graph_expr(expr: &syn::Expr, var_def: &mut HashMap<String,(i32,bool,bool)>,graph: &mut Adjlist, method_map: &HashMap<String,(i32,usize)>,last_node_num: usize){
    // 表达式可能出现多个变量的情况
    match expr {
        // 函数调用
        Expr::Call(exprcall)=> {
            // 放入函数调用的前提是，函数签名中带有hashmap存储的key值
            // signature
            // 对于建图 目前考虑和block类似 开头结尾构建func起点以及终点 对于owner需要考虑特殊标注？
            // 首先建立func节点
            // 对于建立func节点 
            if let Expr::Path(exprpath) = &*exprcall.func{


                let return_value = exprcall.args.len();
                graph.push_func_node(&format!("{}", path_fmt(&exprpath)), true, false, return_value);
                graph.add(graph.len_num()-2, graph.len_num()-1);
                // 1 代表owner； 2代表mut ref ；3代表immutref；接下来是 ref 和 mutability
                
                
                

                for arg in &exprcall.args {
                    graph_expr(arg, var_def, graph, method_map,graph.len_num()-1);
                }

                graph.push_func_node(&format!("{}", path_fmt(&exprpath)), false, true,return_value);
                graph.add(graph.len_num()-2, graph.len_num()-1);
                // 节点构建后遍历其signature
            }
        

        }
        // 方法调用
        Expr::MethodCall(exprmethod) => {
            // methodcall 节点与func call相同 创建methodcall节点 并且遍历其arg
            // method 节点插入
            let method_name = &String::from(format!("{}",exprmethod.method));
            if let Some(method_info) = method_map.get(method_name){
                // method_info.0是self选项 method_info.1是arg number
                // 找到当前method
                // 
                graph.push_method_node(method_name, true, false, method_info.0 ,method_info.1);
                graph.add(graph.len_num()-2, graph.len_num()-1);

                // 获取了当前的method 还需要插入一个self节点？ 隐式调用了self 把self放进去 插入一个self名字的节点
                let mut statement = (0,false,false);
                if method_info.0 == 0 {
                    statement.0 = 1;
                    statement.1 = false;
                    statement.2 = false;
                }else if method_info.0 == 1 {
                    statement.0 = 3;
                    statement.1 = true;
                    statement.2 = false;
                }else if method_info.0 == 2 {
                    statement.0 = 1;
                    statement.1 = false;
                    statement.2 = true;
                }else if method_info.0 == 3 {
                    statement.0 = 2;
                    statement.1 = true;
                    statement.2 = true;
                }
                let var_str = "self".to_string();
                graph.push_node(&statement, &var_str);
                graph.add(graph.len_num()-2, graph.len_num()-1);



                for arg in &exprmethod.args {
                    graph_expr(arg, var_def, graph, method_map,graph.len_num()-1);
                }
                graph.push_method_node(method_name, false, true, method_info.0, method_info.1);
                graph.add(graph.len_num()-2, graph.len_num()-1);
            }


            // 优先处理一个method_call hash表 根据表进行处理

        }
        // todo:: exprindex 数组元素
        Expr::Assign(exprassign) => {
            graph_expr(&exprassign.left.as_ref(), var_def, graph, method_map, graph.len_num()-1);
            graph_expr(&exprassign.right.as_ref(), var_def, graph, method_map,graph.len_num()-1);
        }
        Expr::AssignOp(exprassignop) => {
            graph_expr(&exprassignop.left.as_ref(), var_def, graph, method_map,graph.len_num()-1);
            graph_expr(&exprassignop.right.as_ref(), var_def, graph, method_map,graph.len_num()-1);
        }
        Expr::Binary(exprbinary) => {
            graph_expr(&exprbinary.left.as_ref(), var_def, graph, method_map,graph.len_num()-1);
            graph_expr(&exprbinary.right.as_ref(), var_def, graph, method_map,graph.len_num()-1);
        }
        Expr::Cast(exprcast) => {
            graph_expr(&exprcast.expr.as_ref(), var_def, graph, method_map,graph.len_num()-1);
        }
        Expr::Block(exprblock) => {
            
            graph_block(&exprblock.block, var_def, graph, method_map, last_node_num);

        }
        Expr::Struct(exprstruct) => {
            // struct 表达式与 pat相关
        }
        Expr::Reference(exprreference ) => {
            // 临时变量的产生 在这里会有变量变化的疑问，直接在在这里解开比较合适 解开method call

            // 临时变量 处理三个？path 
            // 有reference证明是存在 &需要跟上&
            // 这个必定是个临时变量 需要修改
            let mut mutable = false;
            if let Some(_) = &exprreference.mutability{
                mutable = true;
            }  
            // pat一个新的 
            match exprreference.expr.as_ref(){
                Expr::Path(exprpath) => {
                    // 获取了新的 mutalble和 ref = true 
                    if let Some(var_name) = exprpath.path.get_ident(){
                        //如果名称在hashset之内就可以进行存储
                        if var_def.contains_key(&String::from(format!("{}",var_name))){
                            // 获取元组 从三个变量中获得 需要push的节点的信息
                            if let Some(_) = var_def.get(&String::from(format!("{}",var_name))) {
                                let mut new_value = (-1,false,false);
                                if mutable{
                                    new_value = (2,true,true);
                                }else{
                                    new_value = (3,true,false);
                                }
                                graph.push_node(&new_value, &String::from(format!("{}",var_name)));
                                graph.add(graph.len_num()-2, graph.len_num()-1);
                            }
                            else { println!("wrong value of hashmap");}
        
                        }
                    } 
                }
                Expr::MethodCall(exprmethodcall) => {
                    // reference 右侧如果是新的method
                    graph_expr(exprreference.expr.as_ref(), var_def, graph, method_map,graph.len_num()-1);
                }
                _ => (),
            }


            // graph_expr(&exprreference.expr.as_ref(), var_def, graph, method_map,graph.len_num()-1);
        }

        Expr::If(exprif) => {
            // 重写

            // if 前的判断语句
            graph_expr (&exprif.cond.as_ref(), var_def, graph, method_map,last_node_num);
            // 建立分支节点

            // block start 前的节点
            let if_start = graph.len_num()-1;

            graph_block(&exprif.then_branch, var_def, graph, method_map, if_start);
            // graph.add(graph.len_num()-2, graph.len_num()-1);
            // 第一个branch的结尾 block节点
            let first_brach_end = graph.len_num()-1;
            
            // else branch后的 expr只可能是if 或者block 
            // TODO 多个if分支会导致出现图变复杂，暂且不考虑 else if 
            if let Some(else_branch_expr) = &exprif.else_branch{
                // 第二个block的前节点是 ifstart节点
                graph_expr(else_branch_expr.1.as_ref(), var_def, graph, method_map,if_start);
                // 第二个branch尾节点
                let second_branch_end = graph.len_num()-1;
                // 创建一个空节点做if 的结尾

                graph.push_block_none();
                // 连接两个block尾节点和最后的空节点
                
                graph.add(second_branch_end, graph.len_num()-1);
                graph.add(first_brach_end, graph.len_num()-1);
            }
            
            


            
        }
        Expr::While(exprwhile) => {

            graph_expr (&exprwhile.cond.as_ref(), var_def, graph, method_map,graph.len_num()-1);
            let v = graph.len_num();
            graph_block(&exprwhile.body, var_def, graph, method_map, graph.len_num()-1);
            let u = graph.len_num()-1;
            graph.add(u, v);

        }
        Expr::ForLoop(exprfor) => {
            // 循环在前后block加入一条返回边
            // pat需要单独解析！
            // Todo: 理解 pat 与siginature关系 
            graph_pat(&exprfor.pat, var_def, graph, last_node_num);
            
            graph_expr (&exprfor.expr.as_ref(), var_def, graph, method_map,graph.len_num()-1);
            let v = graph.len_num();
            graph_block(&exprfor.body, var_def, graph, method_map, graph.len_num()-1);
            let u = graph.len_num()-1;
            graph.add(u, v);
        }
        Expr::Loop(exprloop) => {
            graph_block(&exprloop.body, var_def, graph, method_map,graph.len_num()-1);
        }
        Expr::Let(exprlet) => {
            graph_pat(&exprlet.pat, var_def, graph, graph.len_num()-1);
        }
        Expr::Return(exprreturn) => {
            // Todo 设立return节点 
        }
        // 终结点 变量
        Expr::Path(exprpath) =>{
            // let var = exprpath.path.get_ident();
            
            if let Some(var_name) = exprpath.path.get_ident(){
                //如果名称在hashset之内就可以进行存储
                if var_def.contains_key(&String::from(format!("{}",var_name))){
                    // 获取元组 从三个变量中获得 需要push的节点的信息
                    if let Some(value) = var_def.get(&String::from(format!("{}",var_name))) {
                        graph.push_node(value, &String::from(format!("{}",var_name)));
                        graph.add(graph.len_num()-2, graph.len_num()-1);
                    }
                    else { println!("wrong value of hashmap");}

                }
            }   
        }
        // 其他暂不考虑

        Expr::Closure(exprclosure) => {
            // Todo 
        }
        Expr::Unsafe(exprunsafe) => {
            // Todo
            // graph_block(&exprunsafe.block, var_def, graph, method_map, graph.len_num()-1);
        }
        Expr::Macro(exprmacro) => {
            // 只看println 不管别的 mac path无用
            let mut tokentree_buff = Vec::new();
            let mut first_lit = false;
            for item in exprmacro.mac.tokens.clone() {
                match item {
                    proc_macro2::TokenTree::Punct(punct) => {
                        if (!first_lit) {
                            tokentree_buff.clear();
                            first_lit =true;
                        }else {
                            let mut tokensteram = proc_macro2::TokenStream::new();
                            tokensteram.extend(tokentree_buff);
                            let res: Result<syn::Expr, syn::Error> = syn::parse2(tokensteram);
                            match  res{
                                Ok(exp) => graph_expr(&exp, var_def, graph, method_map, last_node_num),
                                Err(_) => println!("erro macro"),
                            }
                            tokentree_buff = Vec::new();
                        }
                    }
                    _ => {
                        tokentree_buff.push(item);
                    }
                }
            }
            // for 循环结束再做一遍
            let mut tokenstream_buff = proc_macro2::TokenStream::new();
            tokenstream_buff.extend(tokentree_buff);
            let res: Result<syn::Expr, syn::Error> = syn::parse2(tokenstream_buff);
            match res {
                Ok(exp) => graph_expr(&exp, var_def, graph, method_map, last_node_num),
                Err(_) => debug!("Assert macro parse error"),
            }



        }
        _ => {}
    }

} // 


fn graph_stmt(stmt: &syn::Stmt, var_def: &mut HashMap<String,(i32,bool,bool)>, graph: &mut Adjlist,method_map: &HashMap<String,(i32,usize)> , last_node_num: usize){
    //stmt;别名表;图; 表示节点应该插入在哪个节点的后面 ？ 节点后的位置如何判定 连边？
    // 有多个

    // Todo ：varinfo中mut ref的信息没有更正 应该与hashset保存一直
    let mut varloc = VarInfo{Name: None, Mutability:false, Reference:false, Number: 0};

    match stmt{
        // 解析 let 表达式
        Stmt::Local(loc) =>{
            // let 语句 先判断名称后确定是否加入
            let mut pushornot = false;
            let mut var_str = "no".to_string();
            match &loc.pat {
                // owner todo: 这里代码需要简化
                Pat::Ident(patident) => {
                    // 判断是否存在在哈希表
                    if var_def.contains_key(&String::from(format!("{}",patident.ident))){
                        // 确认存在，存入图中
                        // 需要确认节点内部 是否是引用
                        var_str = String::from(format!("{}",patident.ident));
                        pushornot = true;
                        varloc.Name = Some(String::from(format!("{}",patident.ident)));
                        if let Some(_mutable) = &patident.mutability {
                            varloc.Mutability = true;
                        }
                    }
                }
                // reference
                Pat::Reference(patref) => {
                    if let Pat::Ident(patident) = &*patref.pat {
                        if var_def.contains_key(&String::from(format!("{}",patident.ident))){
                            pushornot = true;
                            var_str = String::from(format!("{}",patident.ident));
                            varloc.Name = Some(String::from(format!("{}", patident.ident)));
                            if let Some(_mutable) = &patref.mutability {
                                varloc.Mutability = true;
                            }
                        }
                    }
                }
                _ => println!("Not support stmt")
            }

            // 赋值后面语句的表达式
            // 不能直接expr 与表达式有关
            if let Some((_eq, expr)) = &loc.init {
                
                match expr.as_ref() {
                    Expr::Path(exprpath) =>{
                        // 如果是path就无所谓
                    }
                    // 如果是普通的命名 就不管后面的
                    // 如果是method 直接就call到method 下来
                    Expr::Reference(exprReference) => {
                        // reference 右侧如果也是path就也无所谓
                        match exprReference.expr.as_ref() {
                            Expr::Path(exprpath) =>{
                                // 如果是path就无所谓
                            }
                            _ =>{
                                graph_expr(expr, var_def, graph, method_map,graph.len_num());
                            }
                        }
                    }
                    _=>{
                        graph_expr(expr, var_def, graph, method_map,graph.len_num());
                    }
                }      
            }


            // 应当在hashset 中存储 mutablility 以及 reference内容
            // 节点编号可能会出问题？什么时候push?
            // 是否应当在结束后push/add
            // 还是let声明 不需要存储正在定义的变量 只需要后面出现的变量？
            // 这里需要后面出现的变量名称返回值改名为当前let语句中的identifyname 
            if pushornot {
                // println!("{:?}qweqw",varloc.Name);
                varloc.Number = graph.len_num();
                // todo statement 需要根据Ident/Reference修改吗？
                if let Some(value) = var_def.get(&var_str) {
                    graph.push_node(value, &var_str);
                    graph.add(graph.len_num()-2, graph.len_num()-1);
                }
                else { println!("wrong value of hashmap");}
            }
        },
        Stmt::Semi(expr,_semi) => {
            graph_expr(expr, var_def, graph,method_map,graph.len_num()-1);
        },
        Stmt::Expr(expr) => {
            graph_expr(expr, var_def, graph,method_map,graph.len_num()-1);
        }
        Stmt::Item(item) => {
            // Item fn?
        }



    }





}// 解析普通语句，进行图构建



#[test]
fn synparse_run() {
    // 跑一个case 查看跑出来的东西
    let path_name = "./src/parse/tests/1.rs";

    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);
    // println!("{:?}",content);
    // 没有parsefile？
    let ast = syn::parse_file(&content)
        .expect("ast failed");
    // println!("{:#?}",ast); 打印ast
    // 目前假设函数名字就是main
    // 当前上下文不敏感

    // get hashset 获取别名表
    // Todo ： hashset 需要保存的不只string 还有mut ref等info
    // let mut var_set = HashMap::new();
    // var_set.insert("max".to_string(),);
    // var_set.insert("min".to_string());
    
    let name ="main";

    

    let mut var_set = alias_analysis::create_alias_hashmap(path_name,name);

    let method_map = method_call_names(path_name);
    // 打印别名表
    println!("{:?}",var_set);
    
    // 打印方法表
    // println!("{:?}",method_map);


    // var_set.insert("my_array".to_string(),(1 as i32,false,false));
    // var_set.insert("max".to_string(),(1 as i32,false,false));
    // var_set.insert("min".to_string(),(1 as i32,false,false));
    //let name = String::from("main");
    // 生成并且打印图 获取图的样貌
    let graph = graph_generate(&ast, String::from("main"),&mut var_set, &method_map, &name);
    // 生成csv x / edge 向量
    graph.show();
    
    
}


// 对于单个的code代码，通过graph_generate生成csv x/edge
pub fn csv_creat(code_path: &str, csv_path: &str, funcname: &str)
{
    // 目录是当前cmd下目录相对路径
    // 读取文件
    let mut file = File::open(Path::new(code_path))
        .expect("open file failed ");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let ast = syn::parse_file(&content)
        .expect("ast failed");
    // 图生成
    let method_map = method_call_names(code_path);

    let mut var_set = alias_analysis::create_alias_hashmap(code_path,funcname);
    // let name = String::from("main");
    let graph = graph_generate(&ast, funcname.to_string(),&mut var_set, &method_map, funcname);
    // 已经得到graph之后
    use csv::Writer;
    let xcsv = csv_path.to_string() + "x.csv";
    let edgecsv = csv_path.to_string() + "edge.csv";

    let mut wtr1 = Writer::from_path(&xcsv).expect("read csv wrong");
    let mut wtr2 = Writer::from_path(&edgecsv).expect("read csv wrong");
    // graph.show(); 函数体内部有bug
    // 生成x
    // todo ： 这里的节点有一定问题
    for i in 0..graph.len_num() {
        // 变量名称如何表示？
        let y = graph.vector_x_attribute(i);
        // 暂且不管function call的名字 只考虑变量名相同关系
        // 用hashmap中的顺位表示变量的string有待商榷
        let mut varnumber =0;
        for key in var_set.keys() {
            if key.to_string() == y.0.to_string() {
                break;
            }
            varnumber+= 1;
        }
        let x = (varnumber,y.1,y.2,y.3,-1,y.5,y.6,y.7);        
        wtr1.write_record(&[x.0.to_string(), x.1.to_string(), x.2.to_string(),x.3.to_string(),x.4.to_string(),x.5.to_string(),x.6.to_string(),x.7.to_string()])
            .expect("write_wrong");
    }
    for i in 0..graph.len_num() {
        let edge = graph.vector_edge_attribute(i);
        for e in edge {
            // println!("123123123");
            wtr2.write_record(&[i.to_string(), e.to_string()])
                .expect("write_wrong");
        }
    } 
    
    wtr1.flush().expect("write_wrong");
    wtr2.flush().expect("write_wrong");
    // 创建了两个csv表，一个是x向量，一个是edge向量


    // 输出到csv中
}
#[test]
fn csv_create_test2(){
    let a ="./spider_stackoverflow/src/dataok/code0/0.rs";
    let b = "./spider_stackoverflow/src/dataok/code0/0";
    let c = "f";
    csv_creat(a,b,c);
}

// 输出csv文件
#[test]
fn csv_create_test() 
{
    // 表示图的构建
    // todo: 批处理？生成多个x edge？
    use csv::Writer;
    // 先不考虑多个文件？

    // 读取源代码

    let path_name = "./src/graphcsv/code/test.rs";

    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let ast = syn::parse_file(&content)
        .expect("ast failed");

    // 从ast中获取methodcall
    let method_map = method_call_names(path_name);
    let name ="main";
    let mut var_set = alias_analysis::create_alias_hashmap(path_name,name);
    let name = String::from("main");
    let graph = graph_generate(&ast, String::from("main"),&mut var_set, &method_map, &name);
    // 获取得到cfg
    // 暂时只考虑main函数
    // graph.show();

    let mut wtr1 = Writer::from_path("./src/graphcsv/csv/testx.csv").expect("read_wrong");
    let mut wtr2 = Writer::from_path("./src/graphcsv/csv/testedge.csv").expect("read_wrong");
    // 对graph进行解析 生成 x向量和 edge向量
    // 对每个节点生成x 与edge
    // x输出
    for i in 0..graph.len_num() {
        // 变量名称如何表示？
        let y = graph.vector_x_attribute(i);
        // 暂且不管function call的名字 只考虑变量名相同关系
        // 用hashmap中的顺位表示变量的string有待商榷
        let mut varnumber =0;
        for key in var_set.keys() {
            if key.to_string() == y.0.to_string() {
                break;
            }
            varnumber+= 1;
        }
        let x = (varnumber,y.1,y.2,y.3,-1,y.5,y.6,y.7);        
        wtr1.write_record(&[x.0.to_string(), x.1.to_string(), x.2.to_string(),x.3.to_string(),x.4.to_string(),x.5.to_string(),x.6.to_string(),x.7.to_string()])
            .expect("write_wrong");
    }
    for i in 0..graph.len_num() {
        let edge = graph.vector_edge_attribute(i);
        for e in edge {
            // println!("123123123");
            wtr2.write_record(&[i.to_string(), e.to_string()])
                .expect("write_wrong");
        }
    } 
    
    // wtr.write_record(&["a", "b", "c"]).expect("write_wrong");
    // wtr.write_record(&["x", "2", "z"]).expect("write_wrong");
    wtr1.flush().expect("write_wrong");
    wtr2.flush().expect("write_wrong");
    // 创建了两个csv表，一个是x向量，一个是edge向量

}