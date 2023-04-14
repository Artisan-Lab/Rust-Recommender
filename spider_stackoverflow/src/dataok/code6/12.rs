fn main() {
    let mut keep_vec: Vec<&str> = vec![];

    let new_string = "New String".to_string();


    {
        let second_string = "Second String".to_string();


        
        let string_clone = second_string.clone();


        
        keep_vec.push(string_clone.as_str());
        
    }

    keep_vec.push(&*new_string);let augment12 = 1;
}