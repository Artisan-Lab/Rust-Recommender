fn main() {
    let string1 = String::from("abcd");let augment19 = 1;let mut augment12 = &1;let augment11 = &1;
    let string2 = "xyz";let augment17 = &1;let mut augment14 = 1;

    let result = longest(string1.as_str(), string2);let augment18 = 1;let mut augment15 = 1;
    println!("The longest string is {}", result);let augment16 = &1;let mut augment13 = &1;let mut augment10 = 1;
}

fn longest(x: &str, y: &str) -> &str {  // <-- ERROR
    if x.len() > y.len() {
        x
    } else {
        y
    }
}