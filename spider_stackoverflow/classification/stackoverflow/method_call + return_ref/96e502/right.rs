fn main() {
    let mut s = String::from("Hello World!!");
    try_changes(&mut s);
}

fn try_changes(s: &mut String) {
    let char_h = s.get(0..1).unwrap().to_string();
    s.replace_range(0..1, "h");
    println!("char before {} string after {}", char_h, s);
}