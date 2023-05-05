
struct A{
    a: String,
}

impl A{
    fn return_own(&mut self)-> String{
        &self;
        return self.a;
    }
}

fn main()
{
    
}