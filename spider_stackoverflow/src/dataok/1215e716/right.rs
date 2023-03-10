fn main() {
    let my_bool = true;
    let other = String::from("my_string");
    
    let result;
    let result = if my_bool {
        result = format!("_{}", other);
        result.as_str()
    } else {
        "other"
    };

    println!("{}", result);
}