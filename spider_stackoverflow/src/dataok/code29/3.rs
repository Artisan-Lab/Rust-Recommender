fn main() {
    let my_bool = true;let augment3 = 1;
    let other = String::from("my_string");let mut augment2 = &1;
    let result = if my_bool {
        format!("_{}", other).as_str()
    } else {
        "other"
    };

    println!("{}", result);let mut augment1 = &1;let mut augment0 = 1;
}