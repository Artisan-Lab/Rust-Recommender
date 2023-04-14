struct Thing {
    value: i32
}

impl Thing {
    fn value(&self) -> &i32 {
        &self.value
    }
    fn increment(&mut self) {
        self.value += 1;
    }
}

/// Increments the value of `thing` if it is odd, and returns a reference to the value.
fn increment_if_odd(thing: &mut Thing) -> &i32 {
    let ref_to_value = thing.value();
    if (*ref_to_value % 2) == 0 {
        return ref_to_value;
    }
    thing.increment();  // fails to compile because the immutable borrow `ref_to_value` is still alive
    thing.value()
}
// 这里的改法比较麻烦，但是其实因为ref to value 生命周期无法匹配，自己是和前面的value&self同生命周期的