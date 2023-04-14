struct Thing {
    value: i32
}

impl Thing {
    fn value(&self) -> &i32 {
        &self.value
    }
    fn mut_value(&mut self) -> &mut i32{
        &mut self.value
    }
    fn increment(val: &mut i32) {
        *val += 1;
    }
}

/// Increments the value of `thing` if it is odd, and returns a reference to the value.
fn increment_if_odd<'a>(thing: &'a mut Thing) -> &'a i32 {
    
    let ref_to_value : &'a mut i32 = thing.mut_value();
    if (*ref_to_value % 2) != 0 {
        Thing::increment(ref_to_value);
    }
    ref_to_value
}