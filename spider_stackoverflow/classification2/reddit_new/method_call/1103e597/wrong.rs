struct Person<'person, 'title> {
    name: &'person str,
    title: Option<&'title str>,
}

fn main() {
    
    let mut t1 = Person { name: "Joe" , title: None};
    println!("Name: {}", t1.name);
    
    {
        // made a string as otherwise the lifetime is 'static
        let title = "Dr.".to_string(); 
        t1.title = Some(title.as_str());
        println!("Name: {} {}", t1.title.unwrap(), t1.name);
        t1.title = None;
    }

    println!("Name: {}", t1.name);
}