pub struct Value {
    pub data:f64,
    pub local_grad:f64,
    pub global_grad:f64,
    pub first_child:Option<Box<Value>>,
    pub second_child:Option<Box<Value>>
}

struct Neuron {
    pub w: Vec<value::Value>,
    pub b: value::Value,
    pub relu: bool
} 



fn main()
{
    let mut v = vec![];
    while x.len() != 0 {
            let one = x.remove(0);
    	let two = n.w.remove(0);
    	let val = value::mult(one, two);
    	v.push(val);
    }
}
