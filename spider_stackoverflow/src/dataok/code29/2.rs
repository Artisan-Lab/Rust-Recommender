fn main() {
    let my_bool = true;let augment2 = &1;let mut augment1 = 1;
    let other = String::from("my_string");
    let result = if my_bool {
        format!("_{}", other).as_str()
    } else {
        "other"
    };

    println!("{}", result);let augment0 = &1;
}