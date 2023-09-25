fn main() {
    let mut xs: Vec<&str> = Vec::new();

    let mut s;
    for i in 0..2 
    {
        if i == 0 
        {
            s = String::from("hello");
        }
        else
        {
            s = String::from("bye");
        }
        xs.push(&s);
    }
    
    println!("vector: {:?}", &xs);
}