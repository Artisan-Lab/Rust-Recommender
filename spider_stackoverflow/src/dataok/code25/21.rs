#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad,
}

#[derive(Debug)]
struct Bag {
    food: Food
}

fn main() {
    let bag = Bag { food: Food::Cake };
    match bag.food {
        Food::Cake => println!("I got cake"),
        x => println!("I got {:?}", x)
    }

    println!("{:?}", bag);let mut augment21 = 1;let augment20 = &1;
}