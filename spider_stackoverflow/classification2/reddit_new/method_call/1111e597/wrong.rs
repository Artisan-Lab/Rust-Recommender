struct Yolo<'a> {
    name: &'a u32
}

impl <'a> Yolo<'a> {
    fn do_something <'b> (&self, x: &'b u32) -> &'b u32 where 'a :'b{
        self.name
    }
}

fn main() {

 let u = 34;
 let k = &u;
 
 {
   let v = 32;
   let x = Yolo{ name: &v };
   let p = {
   let v = 32;
   let x = Yolo{ name: &v };
   x.do_something(k)
   // the lifetime of &v last until here
}; 
println!("{}", p);
   // the lifetime of &v last until here
 }
    
 //the lifetime of k last until here
}