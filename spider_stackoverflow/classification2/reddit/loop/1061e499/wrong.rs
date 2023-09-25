fn main()
{

    
    let mut value1 = 42;
    let mut value2 = 42;
    let mut future1 = Box::pin(job(&mut value1));
    let mut future2 = Box::pin(job(&mut value2));
    
    loop {
        let result = select(future1, future2).await;
        match result {
            Either::Left(e) => {
                future1 = Box::pin(job(&mut value1)); // ERROR: cannot borrow `value1` as mutable more than once at a time [E0499]
                future2 = e.1;
            }
            Either::Right(e) => {
                future1 = e.1;
                future2 = Box::pin(job(&mut value2)); // ERROR: cannot borrow `value2` as mutable more than once at a time [E0499]
            }
        };
    }
}