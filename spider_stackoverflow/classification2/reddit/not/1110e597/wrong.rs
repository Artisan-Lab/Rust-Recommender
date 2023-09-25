#[derive(Debug)]
struct S<'a, 'b> {
    pub a: &'a i32,
    pub b: &'b i32
}

fn main() {
    let a = 1;
    
    let mut s: S = S { a: &a, b: &a };
    
    let mut dup_s = 
    {
        let b = 2;
        s = S { a: &a, b: &b };
        s // b is dropped, so s shouldn't be able to be returned
    };

    dup_s = S { a: &a, b: &a }; // bad value of dup_s overwritten immediately

    println!("{:?}", dup_s);
}