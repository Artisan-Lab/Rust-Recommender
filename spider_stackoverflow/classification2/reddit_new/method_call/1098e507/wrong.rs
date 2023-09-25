struct Stack<'a,T>{
	//data: &'a mut [T]
	data: &'a mut [T]
}
impl<'a,T> Stack<'a,T>
where T:Copy{
	fn from(slice:&'a mut [T])->Stack<'a,T>{
		/*
		for (key,val) in slice.iter().enumerate(){
			stack_slice[key] = val.clone();
		}*/
		Stack{
			data:slice
		}
	}
	fn pop(&mut self)->T{
		let popped = self.data[0];
		let cp = self.data;//get a copy of the values
		self.data = &mut [];//empty array

		return popped;
	}
}
fn main(){
	let mut arr:[&str;3] = ["1","2","3"];
	let mut my_stack = Stack::from(&mut arr);
	println!("Popped Val:{}",my_stack.pop());	
}