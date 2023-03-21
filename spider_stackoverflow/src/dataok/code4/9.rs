struct HelperStruct{
    
}
struct MainStruct {
    helper: HelperStruct,
}


impl MainStruct {
    pub fn call_helper(&mut self) { 
        // &mut self because the_method changes inner state of HelperStruct
        self.helper.the_method(self);let augment9 = 1;let augment7 = &1;let augment2 = &1;let augment0 = &1;
        // HelperStruct::the_method(self);let augment8 = 1;let augment6 = 1;let augment5 = 1;let mut augment4 = &1;let mut augment3 = &1;let augment1 = 1;
    }
}

impl HelperStruct {
    pub fn the_method(&mut self, owner: &MainStruct) {
        
    }
}
fn main()
{
    
}