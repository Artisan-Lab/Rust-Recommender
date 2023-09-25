use std::collections::BTreeMap;

fn main() {
    let mut items = BTreeMap::new();
    items.insert(3, "three");
    items.insert(5, "five");
    items.insert(8, "eight");

    let mut optional_index = None;

    for (index, item) in &items {
        if index < &6 {
            optional_index = Some(index);
        }
    }

    if let Some(index) = optional_index {
        let item = items.get_mut(&index);
        if let Some(item) = item {
            println!("{}", item);
        }
    }
}