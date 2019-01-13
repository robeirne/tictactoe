use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;
use std::fmt;
use std::io::{stdin, Stdin, stdout, Write, Error, ErrorKind};
use std::str::FromStr;

fn main() {
    let mut game = Game::new();
    let outcome = game.play_game();
    println!("{}\n{}", game, outcome);
}

#[derive(Debug, Clone)]
struct Game(BTreeMap<Square, Option<Player>>);

impl Game {
    fn new() -> Game {
        let mut game = BTreeMap::new();

        game.insert(Square::A, None);
        game.insert(Square::B, None);
        game.insert(Square::C, None);
        game.insert(Square::D, None);
        game.insert(Square::E, None);
        game.insert(Square::F, None);
        game.insert(Square::G, None);
        game.insert(Square::H, None);
        game.insert(Square::I, None);

        Game(game)
    }

    fn is_complete(&self) -> bool {
        self.0.values()
            .all(|player| player.is_some())    
    }

    fn play_game(&mut self) -> GameOutcome {
        
        let mut i = 0;
        loop {
            let player = match i % 2 {
                0 => Player::X,
                1 => Player::O,
                _ => unreachable!()
            };

            print!("{}\nPlayer {}: ", self, player);
            stdout().flush().expect("Problem writing to stdout!");

            let square = Square::from_input(&stdin());

            let play_outcome = self.execute(player, square);

            match play_outcome {
                PlayOutcome::Next(_) => i += 1,
                PlayOutcome::Draw => break GameOutcome::Draw,
                PlayOutcome::Win(p) => break GameOutcome::Winner(p),
            }
        }
    }

    fn execute(&mut self, player: Player, square: Square) -> PlayOutcome {
        self.0.insert(square, Some(player));

        if self.has_winner() {
            PlayOutcome::Win(player)
        } else if self.is_draw() {
            PlayOutcome::Draw
        } else {
            PlayOutcome::Next(player.next())
        }
    }

    fn has_winner(&self) -> bool {
        let x_squares = self.player_squares(Player::X);
        let y_squares = self.player_squares(Player::O);

        for set in Square::winning_sets() {
            if x_squares == set || y_squares == set {
                return true;
            }
        }

        false
    }

    fn player_squares(&self, player: Player) -> BTreeSet<Square> {
        self.0.iter()
            .filter(|(_, op)| *op == &Some(player))
            .map(|(square, _)| *square)
            .collect()
    }

    fn is_draw(&self) -> bool {
        self.is_complete() && !self.has_winner()
    }
}

#[derive(Debug)]
enum GameOutcome {
    Winner(Player),
    Draw
}

#[derive(Debug)]
enum PlayOutcome {
    Win(Player),
    Draw,
    Next(Player),
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum Square {
    A, B, C, D, E, F, G, H, I,
}

impl Square {
    fn from_input(stdin: &Stdin) -> Square {
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("Problem reading stdin!");
        match Square::from_str(&buf) {
            Ok(square) => square,
            Err(e) => {
                eprintln!("Invalid square: {}", e);
                Square::from_input(stdin)
            }
        }
    }

    fn winning_sets() -> Vec<BTreeSet<Square>> {
        vec![
            FromIterator::from_iter(vec![Square::A, Square::B, Square::C]),
            FromIterator::from_iter(vec![Square::D, Square::E, Square::F]),
            FromIterator::from_iter(vec![Square::G, Square::H, Square::I]),
            FromIterator::from_iter(vec![Square::C, Square::E, Square::G]),
            FromIterator::from_iter(vec![Square::A, Square::E, Square::I]),
        ]
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum Player {
    X,
    O,
}

impl Player {
    fn next(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl FromStr for Square {
    type Err = Error;
    fn from_str(s: &str) -> Result<Square, Error> {
        match s.to_lowercase().trim() {
            "a" => Ok(Square::A),
            "b" => Ok(Square::B),
            "c" => Ok(Square::C),
            "d" => Ok(Square::D),
            "e" => Ok(Square::E),
            "f" => Ok(Square::F),
            "g" => Ok(Square::G),
            "h" => Ok(Square::H),
            "i" => Ok(Square::I),
            _   => Err(Error::from(ErrorKind::InvalidInput))
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let line_separator = "---|---|---\n";

        for (index, (square, o_player)) in self.0.iter().enumerate() {
           match o_player {
               Some(player) => s.push_str(&format!(" {} |", player)),
               None       => s.push_str(&format!(" {} |", square.to_string().to_lowercase()))
           } 

            if (index + 1 ) % 3 == 0 {
                s.pop();
                s.push('\n');
                s.push_str(line_separator);
            }
        }

        let last = s.trim_end_matches(line_separator);

        write!(f, "{}", last)
    }
}

impl fmt::Display for GameOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameOutcome::Draw => write!(f, "{:?}", self),
            GameOutcome::Winner(player) => write!(f, "{} wins!", player),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}