
struct A{
    a: String,
}

impl A{
    fn return_own(&mut self)-> String{
        &self;
        &self.a;
        return self.a;
    }
}

fn main()
{
    
}