use std::collections::HashMap;
struct Person { id: i32 }

fn main() -> std::io::Result<()> {
    let mut first_name_table = HashMap::new();
    let mut last_name_table = HashMap::new();

    let person1 = Person { id: 1};
    let first_name1 = "first1";
    let last_name1 = "last1";

    last_name_table.insert(last_name1, &person1);
    first_name_table.insert(first_name1, person1);
    
    let person2 = Person { id: 2};
    let first_name2 = "first2";
    let last_name2 = "last2";

    last_name_table.insert(last_name2, &person2);let mut augment12 = &1;
    first_name_table.insert(first_name2, person2);

    Ok(())
}