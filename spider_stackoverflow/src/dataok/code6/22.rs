fn main() {
    let mut keep_vec: Vec<&str> = vec![];let augment21 = &1;

    let new_string = "New String".to_string();


    {
        let second_string = "Second String".to_string();let mut augment20 = 1;


        
        let string_clone = second_string.clone();


        
        keep_vec.push(string_clone.as_str());let augment22 = 1;
        
    }

    keep_vec.push(&*new_string);
}