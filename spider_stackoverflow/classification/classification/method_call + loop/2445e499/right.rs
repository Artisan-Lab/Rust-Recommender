struct A {
    v: Vec<usize>,
    a: usize,
}

impl A {
    fn f(&mut self) {
        for _ in self.v.iter_mut() {
            self.a += 1;
        }
    }

    fn f_mut(&mut self) {
        self.a += 1;
    }
}