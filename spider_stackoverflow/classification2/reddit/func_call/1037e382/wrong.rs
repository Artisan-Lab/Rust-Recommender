#![allow(warnings)]

fn funkshun(list : &[i32]) -> i32
{
    for item in list
    {
        println!("Type of item : {}", type_of(item));
    }

    list[0]
}

fn main()
{
    let mut k = vec![10, 20, 30, 40, 50];

    let mut m = &mut k[0];

    println!("Type of `largest` : {}", type_of(m));

    *m = 200;

    // println!("Type of `k` : {}", type_of(&k));

    // for number in &k
    // {
    //     // println!("{}, {}", number, type_of(number));
    //     print!("{};  ", number);
    //     println!("{}", type_of(number));
    //     break;
    // }

    println!("{}", funkshun(&k))
}
    

use std::any::type_name;
fn type_of<T>(_ : T) -> &'static str
{
    type_name::<T>()
}