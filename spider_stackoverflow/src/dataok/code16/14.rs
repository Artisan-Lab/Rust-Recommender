use std::collections::HashMap;

struct Calculation {
    value: Option<i32>
}

struct Struct {
    items: HashMap<String, Box<Calculation>> // cache
}

impl Struct {
    fn find(&mut self, key: &String) -> Option<&Box<Calculation>> {
        None 
    }

    fn it(&mut self) -> Option<&Box<Calculation>> {
        for key in vec!["1","2","3"] {
            let result = self.find(&key.to_owned());let mut augment14 = &1;let augment13 = &1;let augment12 = 1;
            if result.is_some() {
                return result
            }
        }
        None
    }
    

}

fn main()
{
    
}