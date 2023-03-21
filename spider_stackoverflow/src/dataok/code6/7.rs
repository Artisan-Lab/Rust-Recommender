fn main() {
    let mut keep_vec: Vec<&str> = vec![];

    let new_string = "New String".to_string();let augment7 = 1;let augment5 = 1;


    {
        let second_string = "Second String".to_string();let mut augment6 = &1;let augment1 = &1;


        
        let string_clone = second_string.clone();let mut augment3 = 1;let augment2 = &1;


        
        keep_vec.push(string_clone.as_str());
        
    }

    keep_vec.push(&*new_string);let augment4 = 1;let mut augment0 = &1;
}