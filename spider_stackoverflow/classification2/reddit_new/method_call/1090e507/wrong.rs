struct A<T> {
    b: Option<T>
}

impl<T> A<T> {
    fn func(&self) {
        self.b;
    }
}