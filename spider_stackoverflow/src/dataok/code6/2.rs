fn main() {
    let mut keep_vec: Vec<&str> = vec![];

    let new_string = "New String".to_string();


    {
        let second_string = "Second String".to_string();let augment1 = &1;


        
        let string_clone = second_string.clone();let augment2 = &1;


        
        keep_vec.push(string_clone.as_str());
        
    }

    keep_vec.push(&*new_string);let mut augment0 = &1;
}