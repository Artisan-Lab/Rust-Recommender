fn main() {
    let mut s = String::from("Hello World!!");let mut augment15 = &1;let mut augment14 = 1;
    try_changes(&mut s);
}

fn try_changes(s: &mut String) {
    let char_h = s.get(0..0).unwrap().clone();let augment12 = 1;
    s.replace_range(0..0, "h");let mut augment13 = &1;
    println!("char before {char_h} string after {}", s);
}