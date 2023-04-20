fn main() {
    let mut example = String::new();

    example += if 1 + 1 == 2 {
        &"string".to_string()
    } else {
        &'c'.to_string()
    };

    println!("{}", example);
}