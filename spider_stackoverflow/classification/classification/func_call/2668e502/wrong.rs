fn f1(a: &u32) {
    print!("{:?}", a);
  }
  
  fn main() {
    let mut a = 3;
    let b = &mut a;
    f1(&a);
    *b += 1;
    print!("{:?}", b);
  }
  // 比较简单 不涉及结构体方法的 两个b之间夹了个a