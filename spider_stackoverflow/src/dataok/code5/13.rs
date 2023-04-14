fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";let mut augment13 = &1;let mut augment12 = 1;

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {  // <-- ERROR
    if x.len() > y.len() {
        x
    } else {
        y
    }
}