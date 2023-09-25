fn uses_mut(_arg: &mut usize) {
    
}

#[allow(unused_mut)]
fn main() {
    let mut thing: usize = 5;
    
    let mut mut_ref = &mut thing;
    
    let immut_ref: &&mut usize = &mut_ref;
    
    //*mut_ref += 1;
    let _double_mut = &mut mut_ref;
    //uses_mut(mut_ref);
    
    println!("Defence against non-lexical lifetimes {}", immut_ref);
}
