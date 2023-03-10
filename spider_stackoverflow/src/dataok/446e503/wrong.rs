fn main() {
    let mut lst = vec![1, 2, 3];
    let mut x = &mut 0;

    for value in &mut lst {
        x = value;

        *value += 1;
        *x += 1;
    }

    *x += 1;
    println!("{:?}", &lst);
}
