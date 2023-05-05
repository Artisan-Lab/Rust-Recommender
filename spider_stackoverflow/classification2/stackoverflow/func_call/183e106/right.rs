fn main() {
    let string1 = String::from("abcd")  ;
    let string2 = "xyz";

    let x: &str = &string1.as_str();
    let y: &str = &string2;


    let result =    
    if x.len() > y.len() {
        x
    } else {
        y
    };

    println!("The longest string is {}", result);
}
// Why are explicit lifetimes needed in Rust?