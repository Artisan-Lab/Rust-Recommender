fn main() {
    let mut s = String::from("Hello World!!");
    try_changes(&mut s);
}

fn try_changes(s: &mut String) {
    let char_h = s.get(0..0).unwrap().clone();
    s.replace_range(0..0, "h");
    println!("char before {char_h} string after {}", s);
}
// s.get 有&self replace有&mut self 再调用s就不行
// 因为clone了一个&str 所以会复制一个&str &T本身实现了copy

// 注意区分 只有非确定大小类型 比如str 才会 &str clone为 &str 比如&i32 会clone为 i32 
// 这里怎么办 因为没办法判断出来