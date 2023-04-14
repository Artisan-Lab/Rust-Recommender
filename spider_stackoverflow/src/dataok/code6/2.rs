fn main() {
    let mut keep_vec: Vec<&str> = vec![];

    let new_string = "New String".to_string();


    {
        let second_string = "Second String".to_string();


        
        let string_clone = second_string.clone();let augment2 = &1;let mut augment0 = 1;


        
        keep_vec.push(string_clone.as_str());let mut augment1 = 1;
        
    }

    keep_vec.push(&*new_string);
}