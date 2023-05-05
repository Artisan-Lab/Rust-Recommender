

struct Val {
    

}

impl Val {

    fn return_ref(&mut self)-> &i32{
        &1
    }
    fn remove(&mut self, a:&i32) {
        
    }

    fn do_work(&mut self, v: u16) {
        
        loop {
        let r = self.return_ref();
        self.remove(r);
        }


    }
}


fn main() {

}