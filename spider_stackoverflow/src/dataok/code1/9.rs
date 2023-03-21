fn main() {
    let mut s = String::from("Hello World!!");let mut augment9 = &1;
    try_changes(&mut s);let mut augment7 = &1;let augment6 = 1;let augment3 = &1;let augment1 = &1;let augment0 = &1;
}

fn try_changes(s: &mut String) {
    let char_h = s.get(0..0).unwrap().clone();let mut augment2 = &1;
    s.replace_range(0..0, "h");
    println!("char before {char_h} string after {}", s);let mut augment8 = 1;let augment5 = &1;let augment4 = 1;
}