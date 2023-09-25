fn main() {
    let mut x = vec![];
    let mut y = & mut x;
    while let Some(z) = y {
        let some_bool = true;
        if some_bool {
            y = & mut z.next;
        }
    }

}
