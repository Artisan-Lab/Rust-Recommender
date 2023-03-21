fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";let mut augment25 = &1;

    let result = longest(string1.as_str(), string2);let augment26 = 1;let mut augment23 = 1;let augment21 = &1;
    println!("The longest string is {}", result);let mut augment24 = 1;let augment22 = 1;let mut augment20 = 1;
}

fn longest(x: &str, y: &str) -> &str {  // <-- ERROR
    if x.len() > y.len() {
        x
    } else {
        y
    }
}