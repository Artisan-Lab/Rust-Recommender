fn main() {
    let my_bool = true;
    let other = String::from("my_string");let augment16 = &1;
    let result = if my_bool {
        format!("_{}", other).as_str()
    } else {
        "other"
    };

    println!("{}", result);
}