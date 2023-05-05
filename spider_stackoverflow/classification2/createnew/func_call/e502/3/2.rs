fn f(a:&mut i32){
    
}
fn main()
{
    loop {
        let mut a= 3;
        let b=&a;
        f(&mut a);
        b;
    }
}