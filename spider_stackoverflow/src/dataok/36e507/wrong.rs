
enum A {
    Won = 1,
}

struct B {
    result: A,
}

fn print_game(game: &B) {
    println!("{}", game.result as u32);
}

fn main() {
    let game: B = B { result: A::Won };
    print_game(&game);
}