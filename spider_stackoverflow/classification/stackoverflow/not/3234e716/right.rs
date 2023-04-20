fn main() {
    let mut example = String::new();

    if 1 + 1 == 2 {
    example.push_str("string");
} else {
    example.push('c');
}

    println!("{}", example);
}