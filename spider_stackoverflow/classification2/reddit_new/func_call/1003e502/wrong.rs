// The struct should only have an immutable reference
struct Data<'a, T> {
    value: &'a T
} 

fn build<T>(val: &mut T) -> Data<T> {
    // some mutating operation on val here
    Data { value: val }
}

fn main() {
    let mut s = String::from("Some data");
    let data = build(&mut s);

    println!("{}", s); // cannot borrow `s` as immutable because it is also borrowed as mutable

    println!("{}", data.value);
}