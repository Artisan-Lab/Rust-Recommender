fn main() {
    let my_bool = true;let mut augment5 = &1;
    let other = String::from("my_string");let mut augment7 = &1;let augment4 = &1;
    let result = if my_bool {
        format!("_{}", other).as_str()
    } else {
        "other"
    };

    println!("{}", result);let mut augment6 = &1;
}