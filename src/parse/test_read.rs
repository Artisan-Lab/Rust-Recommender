
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[test]
fn generate_test_ast(){
    let mut file = File::open(Path::new("./src/parse/tests/1.rs"))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);
    println!("{:?}",content);
}