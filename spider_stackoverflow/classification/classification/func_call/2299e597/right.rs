// Compiles without errors/warnings.
struct X {payload : i32}

fn main() {
    let pl = X{payload : 44};
    {
        let mut x = &pl;
        println! ("data ={:?}", x.payload);
        {
            let inner = X{payload : 30};
            x = &inner;
            println! ("data ={:?}", x.payload);
            x = &pl;
        }
        println! ("data ={:?}", x.payload);
    }
}