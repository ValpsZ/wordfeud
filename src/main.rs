struct Game<'a> {
    board: &'a Board,
    player1: &'a Player,
    player2: &'a Player,
}

struct Player {
    points: u16,
    inventory: Vec<Tile>,
}

struct Board {
    tiles: [Tile; 225],
}

impl Board {
    fn new_board() -> Self {
        let tiles = [Tile {
            char: ' ',
            points: 1,
        }; 225];
        Board { tiles }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    char: char,
    points: u8,
}

fn main() {
    let board = Board::new_board();
    let player1 = Player {
        points: 0,
        inventory: Vec::new(),
    };
    let player2 = Player {
        points: 0,
        inventory: Vec::new(),
    };
    let game = Game {
        board: &board,
        player1: &player1,
        player2: &player2,
    };
}
