fn main() {
    let mut s = String::from("Hello World!!");let augment11 = &1;let augment10 = &1;
    try_changes(&mut s);let mut augment12 = &1;
}

fn try_changes(s: &mut String) {
    let char_h = s.get(0..0).unwrap().clone();
    s.replace_range(0..0, "h");
    println!("char before {char_h} string after {}", s);
}