fn main() {
    let mut keep_vec: Vec<&str> = vec![];

    let new_string = "New String".to_string();


    {
        let second_string = "Second String".to_string();let mut augment11 = &1;


        
        let string_clone = second_string.clone();let mut augment14 = &1;let mut augment10 = &1;


        
        keep_vec.push(string_clone.as_str());let augment12 = &1;
        
    }

    keep_vec.push(&*new_string);let augment13 = &1;
}