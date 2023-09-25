use std::marker::PhantomData;

#[derive(Clone, Copy)]
struct Phantom<T>(PhantomData<T>);

struct NonCopy;

fn main() {
    {
        // This works as I am expecting, since it doesn't actually contain a NonCopy.
        let a: PhantomData<NonCopy> = PhantomData;
        let b = a;
        let c = a;
    }
    {
        // This doesn't work, although Phantom is just a wrapper and derives Copy.
        let a: Phantom<NonCopy> = Phantom(PhantomData);
        let b = a;
        let c = a;
    }
}