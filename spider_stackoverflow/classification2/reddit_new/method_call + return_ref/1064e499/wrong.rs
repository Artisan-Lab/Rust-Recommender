fn foo(v : &mut Vec<i32>) -> &mut i32{
    {
        if let Some(i) = v.get_mut(0){
            return i;
        }
    }
    return v.get_mut(1).unwrap();
}
fn main()
{
    
}