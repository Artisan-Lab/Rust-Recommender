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
    let bag = Bag { food: Food::Cake };let mut augment17 = &1;let augment16 = 1;let augment15 = &1;
    match bag.food {
        Food::Cake => println!("I got cake"),
        x => println!("I got {:?}", x)
    }

    println!("{:?}", bag);let augment14 = 1;let augment13 = 1;let augment12 = 1;let mut augment11 = &1;let augment10 = &1;
}