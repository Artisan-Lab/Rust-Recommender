struct MyFloat(f64);

struct Point {
    x: MyFloat,
    y: MyFloat,
}

fn foo(x: MyFloat, y: MyFloat) {
    println!("{}", x.0 * y.0);
}

fn bar(obj: &Point) {
    foo(obj.x, obj.y);
}

fn main() {
    bar(& Point { x: MyFloat(2.0), y: MyFloat(5.0) });
}