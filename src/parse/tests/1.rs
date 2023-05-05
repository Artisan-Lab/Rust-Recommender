struct Foo {
    x: i64
}

impl Foo {
    fn get_index(&mut self) -> Option<&i64> {
        if self.x < 0 {
            self.x = 0;
            None
        } else {
            Some(&self.x)
        }
    }
    
    fn use_index(&self, index: &i64) {
        println!("{}", *index);
    }
}

fn main ()
{
    let mut f = Foo{x: 12};
    if let Some(index) = f.get_index() {
        f.use_index(index);    
    }
}
// 这个问题与1003类似 因为返回的一个&i64 与self同生命周期了