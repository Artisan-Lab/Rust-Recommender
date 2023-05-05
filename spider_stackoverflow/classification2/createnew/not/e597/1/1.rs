fn main()
{
    let mut s = "1";
    {
        let tmp = "123".to_string();
        &tmp;
        s = &tmp;
    }
    s;
}