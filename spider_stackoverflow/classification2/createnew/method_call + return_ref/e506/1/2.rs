

struct Val {
    value: i32

}

impl Val {

    fn return_ref(& self)-> &i32{
        &1
    }
    fn remove(&mut self) {
        
    }

    fn do_work(&mut self) {

        loop {
        let r = self.return_ref();
        self.value += 1;
        r;
        }

    }
}


fn main() {

}