struct SingleIterator<T> {   state : Option<T> }
impl<T> SingleIterator<T> {
    fn next123(&mut self) -> Option<T> {
        let result = self.state; // error[E0507]: cannot move out  of self.state which is behind a mutable reference
        self.state = None;
        result
    }
}