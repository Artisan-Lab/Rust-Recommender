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
    let bag = Bag { food: Food::Cake };let augment9 = &1;let augment6 = 1;let augment1 = 1;let mut augment0 = 1;
    match bag.food {
        Food::Cake => println!("I got cake"),
        x => println!("I got {:?}", x)
    }

    println!("{:?}", bag);let mut augment8 = &1;let augment7 = &1;let augment5 = &1;let augment4 = 1;let augment3 = &1;let augment2 = &1;
}