fn main() {
    let my_bool = true;let augment10 = &1;
    let other = String::from("my_string");
    let result = if my_bool {
        format!("_{}", other).as_str()
    } else {
        "other"
    };

    println!("{}", result);
}