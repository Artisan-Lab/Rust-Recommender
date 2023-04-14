fn main() {
    let mut s = String::from("Hello World!!");
    try_changes(&mut s);
}

fn try_changes(s: &mut String) {
    let char_h = s.get(0..0).unwrap().clone();let augment16 = 1;
    s.replace_range(0..0, "h");
    println!("char before {char_h} string after {}", s);
}