fn main()
{
    let a = vec![1, 2, 3];
    for i in &a {
        println!("{}", i);
    }
    for i in &a {
        println!("{}", i);
    }
}