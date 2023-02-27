// 处理一个hash表保存所有method call name self signature  以及 return value


use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::path::Path;
use std::io::{self,prelude::*};


use syn::{self, Stmt, Local, Pat, PatIdent, Expr, MethodTurbofish};
use super::parse_var::{node, VarInfo,stmt_node_type, block_node_type, FuncInfo};
use super::adjlist::Adjlist;

// arg 文件名
// return value methodmap
pub fn method_call_names(path_name: &str) -> HashMap<String,(i32,usize)>{

    let mut methodmap = HashMap::new();

    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let ast = syn::parse_file(&content)
        .expect("ast failed");

    // 获取 AST
    // 从 ast中读取所有 struct 
    // 要求struct 必须在最外层定义 作用域全局 不能定义在内部

    for item in &ast.items{
        
        match item{
            syn::Item::Impl(Itemimpl) => {
                // 搜索method

                for method in &Itemimpl.items{
                    match method {
                        syn::ImplItem::Method(itemMethod) => {
                            // 需要获取的内容： name/ self/ signature number /return value ?
                            
                            let mut method_name = String::from(format!("{}",itemMethod.sig.ident)) ;
                            // signature number
                            let mut arg_number = itemMethod.sig.inputs.len(); 
                            // name
                            let mut method_self = -1;
                            // self
                            for arg_self in &itemMethod.sig.inputs {
                                match arg_self {
                                    syn::FnArg::Receiver(receiver) => {
                                        let mut if_ref = false;
                                        let mut if_mut = false;
                                        
                                        if let Some(_) = receiver.reference {
                                            if_ref = true;
                                        }
                                        if let Some(_) = receiver.mutability {
                                            if_mut = true;
                                        }

                                        if !if_ref && !if_mut{ method_self=0 }; // self
                                        if if_ref && !if_mut{ method_self=1 }; // &self
                                        if !if_ref && if_mut{ method_self=2 }; // mut self
                                        if if_ref && if_mut{ method_self=3 }; // &mut self
                                        break;
                                    }
                                    _ => ()
                                }
                            }

                            // return value ? todo:: 再进行讨论
                            methodmap.insert(method_name.to_string(), (method_self, arg_number));
                        }
                        _ => ()
                    }
                    
                }
            }
            _ => ()
        }
    }
    methodmap

}

#[test]
fn test_create_method_hashmap() {
    // 第一个值为method name 第二个为self 第三个为signature 数量
    let mut methodmap = HashMap::new();
    
    let path_name = "./src/graphcsv/code/test.rs";

    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let ast = syn::parse_file(&content)
        .expect("ast failed");

    // 获取 AST
    // 从 ast中读取所有 struct 
    // 要求struct 必须在最外层定义 作用域全局 不能定义在内部

    for item in &ast.items{
        
        match item{
            syn::Item::Impl(Itemimpl) => {
                // 搜索method

                for method in &Itemimpl.items{
                    match method {
                        syn::ImplItem::Method(itemMethod) => {
                            // 需要获取的内容： name/ self/ signature number /return value ?
                            
                            let mut method_name = String::from(format!("{}",itemMethod.sig.ident)) ;
                            // signature number
                            let mut signature_number = itemMethod.sig.inputs.len(); 
                            // name
                            let mut method_self = -1;
                            // self
                            for arg_self in &itemMethod.sig.inputs {
                                match arg_self {
                                    syn::FnArg::Receiver(receiver) => {
                                        let mut if_ref = false;
                                        let mut if_mut = false;
                                        
                                        if let Some(_) = receiver.reference {
                                            if_ref = true;
                                        }
                                        if let Some(_) = receiver.mutability {
                                            if_mut = true;
                                        }

                                        if !if_ref && !if_mut{ method_self=0 }; // self
                                        if if_ref && !if_mut{ method_self=1 }; // &self
                                        if !if_ref && if_mut{ method_self=2 }; // mut self
                                        if if_ref && if_mut{ method_self=3 }; // &mut self

                                        
                                    }
                                    _ => ()
                                }
                            }

                            // return value ? todo:: 再进行讨论
                            methodmap.insert(method_name.to_string(), (method_self, signature_number));



                        }
                        _ => ()
                    }
                    
                }



            }
            _ => ()
        }
    }
    println!("{:?}", &methodmap);

}