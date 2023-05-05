fn main()
{
    let mut a = vec![1,2];
    match &a {
        &x => (),
        _ => ()
    }
    &a;
}