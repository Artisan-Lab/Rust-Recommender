use std::io;


fn main() {
    let mut words : Vec<String> = Vec::new();

    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            std::result::Result::Ok(_) => {
                let words_line: Vec<&str> = input.split_whitespace().collect();
                match words_line.get(0) {
                    Some(&word) => {
                        words.push(word.to_string());
                    },
                    _ => continue,
                }
            }
            std::result::Result::Err(_) => break
        }
    }

    println!("{:?}", words);
}
