fn main() {
    
    let mut a = Vec::new();
    let mut c = 1;
    for i in 0..2 {
        let b = &mut c;
        &b;
        &b;
        a.push(b);
        
    }
    
    
}