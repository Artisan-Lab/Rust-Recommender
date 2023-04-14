struct A {
    b: B,
}

struct B {
    c: i32,
}

fn f(_a: &A) {}

fn g(_b: &mut B) {}

fn main() {
    let mut foo = A { b: B { c: 2 } };
    let bar = &mut foo.b;
    f(&foo);
    g(bar);
}

// 结构体内部的引用全算在一个家伙头上