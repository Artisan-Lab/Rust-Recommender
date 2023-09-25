fn main() {
    let string = String::from("ABC");
    let mut chars = string.chars();

    for c in chars {
        println!("next {}", chars.next().unwrap());
    }
}