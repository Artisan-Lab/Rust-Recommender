fn main() {
    let mut keep_vec: Vec<&str> = vec![];

    let new_string = "New String".to_string();let mut augment17 = 1;


    {
        let second_string = "Second String".to_string();


        
        let string_clone = second_string.clone();


        
        keep_vec.push(string_clone.as_str());let mut augment16 = &1;
        
    }

    keep_vec.push(&*new_string);let mut augment18 = &1;
}