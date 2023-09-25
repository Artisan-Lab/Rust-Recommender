use std::net::TcpStream;
use std::io::{BufRead, BufReader};

struct Obj {
    reader: BufReader<TcpStream>,
}

impl Obj {
    fn new(stream: TcpStream) -> Obj {
        Obj {
            reader: BufReader::new(stream),
        }
    }

    fn process(&self, line: &str) {
        println!("{}", line);
    }
}

fn main() {
    let stream = match TcpStream::connect("foo.com") {
        Ok(s) => s,
        Err(e) => panic!("Could not connect: {}", e)
    };

    let mut obj = Obj::new(stream);
    let mut it = obj.reader.lines();

    loop {
        match it.next() {
            Some(line) => match line {
                Ok(x) => obj.process(x.as_str()),
                Err(e) => panic!("Could not read: {}", e)
            },
            None => break
        }
    }
}