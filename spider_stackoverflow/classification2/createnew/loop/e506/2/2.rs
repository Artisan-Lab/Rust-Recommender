fn main() {
    
    
    let mut c = 1;
    let a = &mut c;
    for i in 0..2{
        c = *a + 1;
        match c{
            1 => {}
        }
    }

    
}