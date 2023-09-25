struct Thing<'a> {
    f: &'a f64,
  }
  
  trait ThingMaker<'a> {
    fn new_thing(&'a self) -> Box<Thing<'a>>;
  }
  
  struct ThingMakerX<'a> {
    f: &'a f64,
  }
  
  impl<'a> ThingMaker<'a> for ThingMakerX<'a> {
    fn new_thing(&'a self) -> Box<Thing<'a>> {
      panic!("");
    }
  }
  
  fn use_mut<'a>(_thing: &'a mut Thing<'a>) {
    panic!("");
  }
  
  fn use_ref<'a>(_thing: &'a Thing<'a>) { panic!(""); }
  
  fn test(thing_maker_x: &ThingMakerX) {
    let mut thing = thing_maker_x.new_thing();
    use_mut(thing.as_mut());
    use_ref(thing.as_ref());
  }
  
  fn main() {
  }