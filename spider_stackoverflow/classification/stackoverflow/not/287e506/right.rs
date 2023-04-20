//example 5
fn main() {              //line1
    let mut n = 1;       //line2
    let mut nref = &n;   //line3
    n = 4;               //line4
    nref = &n;           //line5
    println!("{}",nref); //line6
}