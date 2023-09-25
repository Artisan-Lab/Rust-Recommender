fn main()
{
    let x = "asdf".to_string();
let y = &x;
drop(x);
println!("{:?}", y);
}