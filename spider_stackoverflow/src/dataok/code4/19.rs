struct HelperStruct{
    
}
struct MainStruct {
    helper: HelperStruct,
}


impl MainStruct {
    pub fn call_helper(&mut self) { 
        // &mut self because the_method changes inner state of HelperStruct
        self.helper.the_method(self);let mut augment19 = &1;let mut augment17 = &1;
        // HelperStruct::the_method(self);let augment18 = 1;let mut augment16 = 1;
    }
}

impl HelperStruct {
    pub fn the_method(&mut self, owner: &MainStruct) {
        
    }
}
fn main()
{
    
}