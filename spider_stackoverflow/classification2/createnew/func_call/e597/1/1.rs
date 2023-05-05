

fn foo(mut s: &mut str) {
    
    loop {
        let mut owner = "123".to_string();
        
        {
            s = &mut owner;
        }
    }
    
}

fn main()
{

}