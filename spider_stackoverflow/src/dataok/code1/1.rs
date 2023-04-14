fn main() {
    let mut s = String::from("Hello World!!");
    try_changes(&mut s);
}

fn try_changes(s: &mut String) {
    let char_h = s.get(0..0).unwrap().clone();let mut augment0 = 1;
    s.replace_range(0..0, "h");let mut augment1 = &1;
    println!("char before {char_h} string after {}", s);
}