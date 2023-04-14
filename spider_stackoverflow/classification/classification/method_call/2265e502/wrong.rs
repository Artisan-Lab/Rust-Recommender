fn first_word(s: &str) -> &str {
    "there"
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("the first word is: {}", word);
}
// 带有s.clear() api中是&mut self，与之前最简单的不太一样