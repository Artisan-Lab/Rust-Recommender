struct HelperStruct{
    
}
struct MainStruct {
    helper: HelperStruct,
}


impl MainStruct {
    pub fn call_helper(&mut self) { 
        // &mut self because the_method changes inner state of HelperStruct
        self.helper.the_method(self);let mut augment15 = 1;let augment10 = &1;
        // HelperStruct::the_method(self);let augment18 = &1;let augment17 = 1;let augment16 = &1;let augment14 = &1;let augment13 = &1;let augment12 = 1;let mut augment11 = &1;
    }
}

impl HelperStruct {
    pub fn the_method(&mut self, owner: &MainStruct) {
        
    }
}
fn main()
{
    
}