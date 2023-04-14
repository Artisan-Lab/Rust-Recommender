fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);let augment0 = &1;
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {  // <-- ERROR
    if x.len() > y.len() {
        x
    } else {
        y
    }
}