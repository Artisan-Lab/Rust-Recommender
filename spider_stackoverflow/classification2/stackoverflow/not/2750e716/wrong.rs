fn main()
{
    let mut v2 = &vec![1,2,3];

let tv = &vec![2,3,4];
v2 = tv;

// different from
v2 = &vec![2, 3, 4];  // uncomment this will error

println!("{:?}", v2);
}