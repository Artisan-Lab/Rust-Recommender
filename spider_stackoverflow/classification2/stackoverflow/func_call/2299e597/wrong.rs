struct X {payload : i32}

fn main() {
    let pl = X{payload : 44};
    {
        let mut x = &pl;
        {
            let inner = X{payload : 30};
            let tmp = std::mem::replace(&mut x, &inner);
            println! ("data ={:?}", x.payload);
            let _f = std::mem::replace(&mut x, &tmp);
        }
        println! ("data ={:?}", x.payload);
    }
}