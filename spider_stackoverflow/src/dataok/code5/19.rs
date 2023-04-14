fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";let mut augment19 = &1;let mut augment16 = &1;

    let result = longest(string1.as_str(), string2);let augment18 = &1;
    println!("The longest string is {}", result);let mut augment17 = &1;
}

fn longest(x: &str, y: &str) -> &str {  // <-- ERROR
    if x.len() > y.len() {
        x
    } else {
        y
    }
}