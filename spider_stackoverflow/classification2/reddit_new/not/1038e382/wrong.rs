fn example_1()
{
    let mut array = [0, 1, 2, 3, 4, 5, 6, 8, 9, 10];
    let foo = &mut array[3..5];

    {
        let bar = foo;
        bar[0] = -1234;
    }

    foo[0] += 1;

    println!("{:?}", array);
}