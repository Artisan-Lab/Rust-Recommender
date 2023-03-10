enum X {
    X1(String),
    X2(String),
}

fn increment_x(x: X) -> X {
    match x {
        X::X1(s) => X::X2(s),
        X::X2(s) => X::X1(s),
    }
}
fn main(){
    
}