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
    let bag = Bag { food: Food::Cake };let mut augment27 = 1;let augment24 = &1;let mut augment22 = &1;
    match bag.food {
        Food::Cake => println!("I got cake"),
        x => println!("I got {:?}", x)
    }

    println!("{:?}", bag);let augment28 = 1;let mut augment26 = &1;let augment25 = 1;let augment23 = 1;let mut augment21 = 1;let augment20 = &1;
}