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
    let bag = Bag { food: Food::Cake };let augment1 = 1;let mut augment0 = 1;
    match bag.food {
        Food::Cake => println!("I got cake"),
        x => println!("I got {:?}", x)
    }

    println!("{:?}", bag);
}