struct HelperStruct{
    
}
struct MainStruct {
    helper: HelperStruct,
}


impl MainStruct {
    pub fn call_helper(&mut self) { 
        // &mut self because the_method changes inner state of HelperStruct
        self.helper.the_method(&self);
        // HelperStruct::the_method(self);
    }
}

impl HelperStruct {
    pub fn the_method(&mut self, owner: &MainStruct) {
        
    }
}
fn main()
{
    
}

// 这个问题调用比较复杂，第一个immutable产生在 self， 第二个immutable在themethod中调用，中间的mutable就是the method中的&mut self 因为都是self，都是同一个引用