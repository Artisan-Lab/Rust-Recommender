struct Hold<'a> {
    owned: Vec<i32>,
    borrowed: Vec<&'a i32>
  }
  fn main() {
  
      let a = vec![1,2,3];
      let b = vec![&a[0], &a[1], &a[2]];
  
      let hold = Hold {
        owned: a,
        borrowed: b
      };
  }