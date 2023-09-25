use std::collections::HashMap;

trait MyTrait {
    fn test(&self, map: &mut HashMap<String, Box<dyn MyTrait>>);
}

struct TraitImpl {}

impl MyTrait for TraitImpl {
    fn test(&self, _map: &mut HashMap<String, Box<dyn MyTrait>>) {}
}

fn main() {
    let mut map: HashMap<String, Box<dyn MyTrait>> = HashMap::new();
    map.insert("key".to_string(), Box::new(TraitImpl {}));

    let val = match map.get("key") {
        Some(ref value) => {
            let impl_val = *value;
            Some(impl_val)
        }
        None => None,
    };

    match val {
        Some(ref value) => {
            let impl_val = *value;
            impl_val.test(&mut map);
        }
        None => (),
    };
}
