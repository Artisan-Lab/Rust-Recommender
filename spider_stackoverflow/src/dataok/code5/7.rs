fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";let augment4 = &1;

    let result = longest(string1.as_str(), string2);let mut augment6 = 1;
    println!("The longest string is {}", result);let mut augment7 = &1;let augment5 = 1;
}

fn longest(x: &str, y: &str) -> &str {  // <-- ERROR
    if x.len() > y.len() {
        x
    } else {
        y
    }
}