fn main() {
    let mut keep_vec: Vec<String> = vec![];

    let new_string = "New String".to_string();

    // create a new string inside an expression
    // and push it into the keep_vec
    {
        let second_string = "Second String".to_string();
        keep_vec.push(second_string);
    }

    keep_vec.push(new_string);
}