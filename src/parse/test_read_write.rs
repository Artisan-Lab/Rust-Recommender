
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::parse_var;

#[test]
fn generate_test_ast(){
    let mut file = File::open(Path::new("./src/parse/tests/1.rs"))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);
    println!("{:?}",content);
}


#[test]
fn write_all_csvs(){
    // 读取所有的数据增强文件，并且把所有生成的embeddeding都放入codei文件夹下
    // 读取codei 文件夹下内容
    // "./spider_stackoverflow/src/dataok/codei/i.rs"
    let baseadd1 ="./spider_stackoverflow/src/dataok/code".to_string();

    let baseadd2 = ".rs".to_string();

    let baseadd3 = "/name.txt".to_string();
    
    let code_nums = 30;

    for i in 27..code_nums+1{
        let mut a = i.to_string();

        for j in 0..30{
            let code_path = baseadd1.to_string() + &i.to_string() + "/" +&j.to_string()+ &baseadd2 ;  
            let csv_path = baseadd1.to_string() + &i.to_string() + "/" +&j.to_string();
            
            let name_path = baseadd1.to_string() + &i.to_string() + "/" +&baseadd3;

            let mut file = File::open(Path::new(&name_path))
                .expect("open name.txt failed ");
            let mut name_func = String::new();
            file.read_to_string(&mut name_func);

            parse_var::csv_creat(&code_path, &csv_path, &name_func);

            // println!("{}",csv_path);


        }

        

    }


}

