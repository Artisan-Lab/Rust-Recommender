struct MyStruct { s: u32 }

fn main() {
    
    let mut a = vec![1];
    let x = &a;
    {
        a;
        
    }
    x;
    
}