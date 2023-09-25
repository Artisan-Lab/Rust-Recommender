// The real pain is actually no GATs
trait Stuff<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = &'a mut Self::Item>;

    fn iter_mut(&'a mut self) -> Self::Iter;
}

struct VecStuff<T>(Vec<T>);

impl<'a, T: 'a> Stuff<'a> for VecStuff<T> {
    type Item = T;
    type Iter = std::slice::IterMut<'a, T>;

    fn iter_mut(&'a mut self) -> Self::Iter {
        self.0.iter_mut()
    }
}

fn test<'a, T: Stuff<'a>>(stuff: &'a mut T) {
    for _ in 0..10 {
        stuff.iter_mut();
    }
}