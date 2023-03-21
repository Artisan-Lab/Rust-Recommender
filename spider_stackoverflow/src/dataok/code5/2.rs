fn main() {
    let string1 = String::from("abcd");let augment2 = &1;
    let string2 = "xyz";let augment0 = 1;

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);let augment1 = &1;
}

fn longest(x: &str, y: &str) -> &str {  // <-- ERROR
    if x.len() > y.len() {
        x
    } else {
        y
    }
}