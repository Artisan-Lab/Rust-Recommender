use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let mut map = HashMap::new();

    let f = File::open("text.txt").expect("File not found");
    let mut file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        for word in l.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    } 
    println!("{:?}", map);
}