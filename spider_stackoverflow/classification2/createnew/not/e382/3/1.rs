struct MyStruct { s: u32 }

fn main() {
    
    let mut a = vec![1];
    
    {
        &a;
        a;
        
    }
    a[0];
    
}