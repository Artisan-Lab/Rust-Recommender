struct Inner {
    some_val: usize,    
    }
    impl Inner {
    pub fn new() -> Self {
    Self {
    some_val: 10,
            }
        }
    }
    struct Outer {
    id: usize,
    inner: Option<Inner>,
    }
    impl Outer {
    pub fn new() -> Self {
    Self {
    id: 1,
    inner: None,
            }
        }
    pub fn fill(&mut self) -> &Inner {
    let obj = Inner::new();
    self.inner = Some(obj);
    &self.inner.unwrap()
        }
    pub fn get_inner(&self) -> &Inner {
    &self.inner.as_ref().unwrap()
        }
    pub fn update_inner(&mut self) -> &Inner {
    self.inner.unwrap().some_val = 20;
    &self.inner.as_ref().unwrap()
        }
    }
    fn main() {
    things();
    }
    fn things() {
    let outer = Outer::new();
    }