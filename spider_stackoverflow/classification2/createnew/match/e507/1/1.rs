fn main()
{
    let mut a = vec![1,2];
    &a;
    match &a {
        &x => (),
        _ => ()
    }
    //a;
}