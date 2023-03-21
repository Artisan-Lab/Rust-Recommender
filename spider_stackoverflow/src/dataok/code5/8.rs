fn main() {
    let string1 = String::from("abcd");let mut augment6 = &1;let augment5 = 1;let augment2 = &1;
    let string2 = "xyz";let mut augment7 = 1;let mut augment4 = &1;let augment3 = &1;let augment0 = 1;

    let result = longest(string1.as_str(), string2);let mut augment8 = &1;
    println!("The longest string is {}", result);let augment1 = &1;
}

fn longest(x: &str, y: &str) -> &str {  // <-- ERROR
    if x.len() > y.len() {
        x
    } else {
        y
    }
}