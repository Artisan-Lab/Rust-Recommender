#![allow(unused_variables)]
#![allow(dead_code)]

use std::io::Error;
use std::collections::HashMap;
use std::fs::File;

//The idea behind `get_value` function is this:
//   Get me a mutable File object associated with a key.
//   In case, key does not exist, create the file and 
//   give me mutable access to the newly created file
//   So unless an `Error` occurs, I should always get 
//   a mut File object.
fn get_value(map: &mut HashMap<i32, File>, key: i32) -> Result<&mut File, Error> {
    {
        if let Some(val) = map.get_mut(&key) {
            return Ok(val);
        }
    }
    
    let new_value = get_default_value(key)?;
    
    map.insert(key, new_value);
    Ok(map.get_mut(&key).unwrap())
}

//The default value generation could return an Error
fn get_default_value(key: i32) -> Result<File, Error> {
    //Assume some logic to open/create a file based on key object
    let f = File::open(format!("/some/path/{}", key))?;
    Ok(f)
}


fn main() {
}