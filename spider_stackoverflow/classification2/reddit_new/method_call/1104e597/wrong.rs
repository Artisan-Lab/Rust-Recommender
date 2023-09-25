use std::collections::HashMap;

fn main() {
let mut some_hashmap: HashMap<(&str, &str), Vec<String>> = HashMap::new();
    for _ in 0..5{
    let word = "helloworld".to_string();
    let key = (&word[..2],&word[3..]);
    if let Some(_)= some_hashmap.remove( &key) {}
    }
}