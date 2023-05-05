

struct Val {
    

}

impl Val {

    fn return_ref(& self)-> &i32{
        &1
    }
    fn remove(&mut self,a:&i32) {
        
    }

    fn do_work(&mut self) {
        
        loop {
        let r = self.return_ref();
        self.remove(r);
        }


    }
}


fn main() {

}