fn main() {
    let mut s = String::from("Hello World!!");let augment11 = &1;let augment10 = &1;
    try_changes(&mut s);let augment15 = 1;let mut augment12 = &1;
}

fn try_changes(s: &mut String) {
    let char_h = s.get(0..0).unwrap().clone();let mut augment16 = 1;
    s.replace_range(0..0, "h");let mut augment14 = 1;let mut augment13 = &1;
    println!("char before {char_h} string after {}", s);
}