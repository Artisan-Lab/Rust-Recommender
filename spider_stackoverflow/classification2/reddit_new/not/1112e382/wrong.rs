struct Player {
    sign: char
}

struct Game<'a> {
    players: [Player; 2],
    current_player: Option<&'a Player>
}

impl Player {
    fn new(sign: char) -> Self {
        Player {
            sign
        }
    }
}

impl<'a> Game<'a> {

    fn new123() -> Self {
        let player1 = Player::new('X');
        let player2 = Player::new('O');
        Game {
            players: [player1, player2],
            current_player: Some(&player1)
        }
    }
}

fn main() {
}
