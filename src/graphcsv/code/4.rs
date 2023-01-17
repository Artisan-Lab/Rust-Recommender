// e505 
struct Value {}

fn borrow(val: &Value) {}

fn eat(val: Value) {}

fn main() {
    let x = Value{};
    let _ref_to_val = &x;
    eat(x);
    borrow(_ref_to_val);
}