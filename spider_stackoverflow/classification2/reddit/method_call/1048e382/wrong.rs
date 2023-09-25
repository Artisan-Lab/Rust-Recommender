fn main() {
    let c = Complex::new();
    let d = Complex{
        r:5.,c:5.,
    };
    let q = c + d;
    d.print();
}