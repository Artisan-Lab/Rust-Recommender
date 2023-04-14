

use std::io::Read;
use std::path::Path;
use std::fs::File;
use syn::{self, Stmt, Local, Pat, PatIdent, Expr, Type};
use super::parse_var::{node, VarInfo,stmt_node_type, block_node_type, FuncInfo};
use super::adjlist::Adjlist;

use std::collections::HashMap;

fn alias_name_expr(expr: &syn::Expr, let_var: &mut (i32, bool, bool, String), var_map: &mut HashMap<String, usize>, graph: &mut Adjlist){
    // 解析expr 获取letvar内容
    match expr{
        Expr::Reference(exprRef)=> {
            let_var.1 = true;
            if let Some(_) = &exprRef.mutability {
                let_var.2 = true;
            }
        }
        Expr::Block(expr_block) => {
            for stmt in &expr_block.block.stmts{
                alias_name_stmt(stmt, graph, var_map);
            }

        }
        // Todo struct
        Expr::Struct(exprstruct) => {

        }
        Expr::Macro(exprmacro) => {}

        // let some(x) = ? 与for循环中存在变量的定义
        // let for 中具有相同问题 定义的变量需要判断后方的expr，
        // TODO 暂且不支持tuple 
        Expr::Let(exprlet) => {
            let mut let_var2 = (0, false, false, "for".to_string());
            let mut tuple_flag = false;
            alias_name_expr(&exprlet.expr, &mut let_var2, var_map, graph);
            // 此时let_var2 已经构建好了ref

            match &exprlet.pat{
                Pat::Ident(patident) => {
                    let_var2.3 = String::from(format!("{}", patident.ident));
                    if let Some(_mutable) = &patident.mutability {
                        let_var2.2 = true;
                    }
                },
                Pat::Reference(pat_reference) => {
                    if let Pat::Ident(pat_ident) = &*pat_reference.pat {

                        let_var2.3 = String::from(format!("{}", pat_ident.ident));
                        if let Some(_mutable) = &pat_reference.mutability {
                            let_var2.2 = true;
                        }
                    }
                },
                // 对于tuple只需要递归下去直到 ident/ref 就可以获取并创建了
                Pat::Tuple(pattuple) => {
                    alias_name_tuple_expr(pattuple, &let_var, var_map, graph);
                    tuple_flag = true;
                }
                _ => {
                    
                }
            }
            // 如果不是tuple 才可以进行加入，否则tuple需要向下递归在结尾加入
            if !tuple_flag{
                if let_var2.1 {
                    if let_var2.2 { // mut
                        let_var2.0 = 2;
                    } else {
                        let_var2.0 =3 ;
                    }
                }else {
                    let_var2.0 = 1; 
                }
                var_map.insert(String::from(format!("{}", let_var2.3)), graph.len_num());
                graph.push_node(&(let_var2.0,let_var2.1,let_var2.2), &let_var2.3);
            }

        }
        Expr::ForLoop(expr_for) => {
            // for loop 
        }
        _=>()
    }
}

// pat + expr 因为tuple导致的递归循环问题，所以只能在 ident/ref 结尾而tuple需要继续递归 
// 此时因为expr已经解析过，已经获得了后续信息，只需解析pat内部即可
fn alias_name_tuple_expr(tuple:&syn::PatTuple,let_var: &(i32, bool, bool, String), var_map: &mut HashMap<String, usize>, graph: &mut Adjlist)
{
    // todo: tuple内容需要做特殊处理吗？
    for element in &tuple.elems {
        // pat 可能是tuple 可能不是
        let mut let_var2 = (let_var.0, let_var.1, let_var.2, String::from(format!("{}", let_var.3)));
        let mut tuple_flag = false;
        match  &element{
            Pat::Ident(patident) => {
                let_var2.3 = String::from(format!("{}", patident.ident));
                if let Some(_mutable) = &patident.mutability {
                    let_var2.2 = true;
                }
            },
            Pat::Reference(pat_reference) => {
                if let Pat::Ident(pat_ident) = &*pat_reference.pat {

                    let_var2.3 = String::from(format!("{}", pat_ident.ident));
                    if let Some(_mutable) = &pat_reference.mutability {
                        let_var2.2 = true;
                    }
                }
            },
            // Todo tuble
            Pat::Tuple(pattuple) => {
                tuple_flag = true;
                alias_name_tuple_expr(pattuple, let_var, var_map, graph);
            }
            _ => {}
        }
        if !tuple_flag{
            if let_var2.1 {
                if let_var2.2 { // mut
                    let_var2.0 = 2;
                } else {
                    let_var2.0 =3 ;
                }
            }else {
                let_var2.0 = 1; 
            }
            var_map.insert(String::from(format!("{}", let_var2.3)), graph.len_num());
            graph.push_node(&(let_var2.0,let_var2.1,let_var2.2), &let_var2.3);
        }

    }

}



// todo : 是否需要一个保存出现过别名的hashmap用来记录所有的变量以及标号？
fn alias_name_stmt(stmt: &syn::Stmt, graph: &mut Adjlist, var_map: &mut HashMap<String,usize>) {
    // 搜寻所有的let语句 以及会出现定义名称的迭代器等，找到所有名称 获得其属性，插入graph
    // 实质是搜索所有let语句，找到其变量内容意义
    
    // 通过let后的表达式判断当前的 let语句创建的变量是什么样的变量
    // 第0个变量表示var当前属于 owner/staref/mutref 第1个变量表示是否是ref 第2个变量表示是否是mut 第3个表示变量名称
    // 结构体暂且没考虑
    let mut let_var = (0, false, false, "for".to_string());
    let mut tuple_flag = false;
    match stmt {
        Stmt::Local(loc) => {

            // 初始化后的expr 中获取letvar信息
            if let Some((_, expr))= &loc.init {
                alias_name_expr(expr, &mut let_var, var_map, graph);
            }
            match &loc.pat{
                // TODO 为了满足tuple 重写 这一部分
                Pat::Ident(pat_ident) => {
                    let_var.3 = String::from(format!("{}", pat_ident.ident));
                    if let Some(_mutable) = &pat_ident.mutability {
                        let_var.2 = true;
                    }
                },

                Pat::Reference(pat_reference) => {
                    if let Pat::Ident(pat_ident) = &*pat_reference.pat {

                        let_var.3 = String::from(format!("{}", pat_ident.ident));
                        if let Some(_mutable) = &pat_reference.mutability {
                            let_var.2 = true;
                        }
                    }
                },
                Pat::Tuple(pattuple) => {
                    alias_name_tuple_expr(pattuple, &let_var, var_map, graph);
                    tuple_flag = true;
                }
                _ => ()
            
            }
            if !tuple_flag {
                // 插入节点
                // ref
                if let_var.1 {
                    if let_var.2 { // mut
                        let_var.0 = 2;
                    } else {
                        let_var .0 =3 ;
                    }
                }else {
                    let_var.0 = 1; 
                }

                var_map.insert(String::from(format!("{}", let_var.3)), graph.len_num());
                graph.push_node(&(let_var.0,let_var.1,let_var.2), &let_var.3);
            }

            

        }
        Stmt::Semi(expr, _) =>{
            alias_name_expr(expr, &mut let_var, var_map, graph);
        }
        Stmt::Expr(expr) => {
            alias_name_expr(expr, &mut let_var, var_map, graph);
        }
        _ => ()
    }

}   



fn alias_map_genarate(ast: &syn::File, funcname: &str) -> Adjlist {
    // graph 存储别名关系 连边
    let mut graph = Adjlist::new();
    // varmap负责查找variable对应名称的graph节点序号
    let mut var_map = HashMap::new();
    // 放入一个空节点
    var_map.insert("for".to_string(), 0); // 0节点是空的，什么也没有
    graph.push_node(&(0,false,false), &"for".to_string());


    // 与parsevar相同搜索ast 
    // todo 是否需要扫两遍？第一遍扫所有的
    for item in &ast.items{ 
        // 传入graph
        match item{
            syn::Item::Fn(func) => {
                // println!("{:?}",func.sig.ident);

                // Todo：for arg in &func.sig.inputs {} 暂时先不看main所有的signiture
                // todo 先只分析main函数
                // todo 暂且不考虑结构体
                if func.sig.ident == funcname{ //是否会出现情况导致无法进入分支条件？
                    // println!("123");
                    // 考虑signature
                    

                    for arg in &func.sig.inputs {
                        let mut let_var = (0, false, false, "for".to_string());
                        match arg {
                            syn::FnArg::Typed(pattyped) => {
                                // patident 是名字
                                match &*pattyped.pat{
                                    Pat::Ident(patident) => {
                                        let_var.3 = (String::from(format!("{}", patident.ident)));
                                    }
                                    _ => {}
                                } 
                                match &*pattyped.ty{
                                    Type::Reference(TyRef) =>{
                                        let_var.1 = true;
                                        
                                        if let Some(_mutability) = &TyRef.mutability{//mutref
                                            let_var.0 = 3;
                                            let_var.2 =true;
                                        }else {//staticref
                                            let_var.2 = false;
                                            let_var.0 = 2;
                                        }
                                    }
                                    Type::Path(typa) => {
                                        let_var.0 = 1;
                                        let_var.1 = false;
                                        let_var.2 = false;

                                    }
                                    _ =>{}
                                }
                            }
                            _ => {}
                        }

                    } 


                    for stmt in &func.block.stmts {    
                        // 从let 开始加入所有语句 主要是搜索所有的let语句，找到所有的定义变量，再进行加边
                        // 先把所有的出现的名称加入varmap中
                        alias_name_stmt(stmt, &mut graph, &mut var_map);
                        
                    }
                }

            }
            syn::Item::Impl(Itemimpl) =>{
                for method in &Itemimpl.items{
                    match method {
                        syn::ImplItem::Method(itemMethod) => {
                            if itemMethod.sig.ident == funcname.to_string() {
                                graph.push(node { stmt:Some(stmt_node_type::Function(FuncInfo{Name: Some(funcname.to_string()), arg_number: 0 , Number: graph.len_num(), Start:true, End:false, method_call: -1})) , block: None });
                                for arg in &itemMethod.sig.inputs {
                                    let mut let_var = (0, false, false, "for".to_string());
                                    match arg {
                                        syn::FnArg::Typed(pattyped) => {
                                            // patident 是名字
                                            match &*pattyped.pat{
                                                Pat::Ident(patident) => {
                                                    let_var.3 = (String::from(format!("{}", patident.ident)));
                                                }
                                                _ => {}
                                            } 
                                            match &*pattyped.ty{
                                                Type::Reference(TyRef) =>{
                                                    let_var.1 = true;
                                                    
                                                    if let Some(_mutability) = &TyRef.mutability{//mutref
                                                        let_var.0 = 3;
                                                        let_var.2 =true;
                                                    }else {//staticref
                                                        let_var.2 = false;
                                                        let_var.0 = 2;
                                                    }
                                                }
                                                Type::Path(typa) => {
                                                    let_var.0 = 1;
                                                    let_var.1 = false;
                                                    let_var.2 = false;

                                                }
                                                _ =>{}
                                            }
                                        }
                                        _ => {}
                                    }
                                } 

                                // signature

                            }
                        }
                        
                        _ => ()
                    }
                    
                }
            }

            _ => () 
        }
    }



    graph
}

pub fn create_alias_hashmap(path_name: &str, funcname: &str) ->HashMap<String,(i32,bool,bool)>{
    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);

    let ast = syn::parse_file(&content).expect("ast failed");
    // 需要一个邻接表图表示？还是一个简单的并查集？
    // 最后返回一个hashmap?

    let graph = alias_map_genarate(&ast,funcname);
    
    // 解析graph 生成hashmap

    let mut Varmap = HashMap::new();
    for i in 0..graph.len_num() {
        if let Some(stmtnode) = &graph.heads_list[i].data.stmt {
            match  stmtnode {
                // 新建加入节点
                stmt_node_type::Owner(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        Varmap.insert(a.to_string(), (1 as i32, false, false));
                    }
                    
                }
                stmt_node_type::MutRef(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        Varmap.insert(a.to_string(), (2 as i32, varinfo.Reference, varinfo.Mutability));
                    }
                }
                stmt_node_type::StaticRef(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        Varmap.insert(a.to_string(), (3 as i32, varinfo.Reference, varinfo.Mutability));
                    }
                }
                _ => ()
                
            }
        }

    }

    Varmap
}


#[test]
fn test_create_alias_hashmap()
{
    let mut file = File::open(Path::new("./src/graphcsv/code/2.rs"))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);

    
    let ast = syn::parse_file(&content).expect("ast failed");

    // 需要一个邻接表图表示？还是一个简单的并查集？
    // 最后返回一个hashmap?
    let name = "main";
    let graph = alias_map_genarate(&ast,name);
    
    graph.show();

    // 根据图 创建所需要的别名表



}

