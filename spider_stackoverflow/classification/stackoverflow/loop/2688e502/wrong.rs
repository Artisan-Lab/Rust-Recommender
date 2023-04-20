use std::io;


fn main() {
    let mut words = Vec::new();

    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            std::result::Result::Ok(_) => {
                let words_line: Vec<&str> = input.split_whitespace().collect();
                match words_line.get(0) {
                    Some(&word) => {
                        words.push(word.clone());
                    },
                    _ => continue,
                }
            }
            std::result::Result::Err(_) => break
        }
    }

    println!("{:?}", words);
}

// 还是与loop有关系， word.clone是创建了一个&str 并且存在了word里面