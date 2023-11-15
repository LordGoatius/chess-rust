use std::{fmt::Display, str::FromStr};
use std::io::{self, Write};
use chess::{Game, Square, Piece, Color, ChessMove, Rank, GameResult};

pub struct MyGame(pub Game);

impl Display for MyGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_str: String = "   ┌───┬───┬───┬───┬───┬───┬───┬───┐\n".to_owned();

        for i in (1..=8).rev() {
            let mut to_append: String = format!(" {i} │");
            for j in 'a'..='h' {
                let piece = self.0.current_position().piece_on(Square::from_str(&format!("{j}{i}")[..]).unwrap());
                let color = self.0.current_position().color_on(Square::from_str(&format!("{j}{i}")[..]).unwrap());
                match color {
                    Some(Color::White) => {
                        match piece {
                            Some(Piece::Pawn)   => to_append.push_str(" 󰡙 │"),
                            Some(Piece::King)   => to_append.push_str(" 󰡗 │"),
                            Some(Piece::Queen)  => to_append.push_str(" 󰡚 │"),
                            Some(Piece::Knight) => to_append.push_str(" 󰡘 │"),
                            Some(Piece::Bishop) => to_append.push_str(" 󰡜 │"),
                            Some(Piece::Rook)   => to_append.push_str(" 󰡛 │"),
                            None                => to_append.push_str("   │"),
                        }
                    },
                    Some(Color::Black) => {
                        match piece {
                            Some(Piece::Pawn)   => to_append.push_str("  │"),
                            Some(Piece::King)   => to_append.push_str("  │"),
                            Some(Piece::Queen)  => to_append.push_str("  │"),
                            Some(Piece::Knight) => to_append.push_str("  │"),
                            Some(Piece::Bishop) => to_append.push_str("  │"),
                            Some(Piece::Rook)   => to_append.push_str("  │"),
                            None                => to_append.push_str("   │"),
                        }
                    },
                    None => to_append.push_str("   │"),
                }
            }
            to_append.push('\n');
            board_str.push_str(&to_append[..]);
            if i != 1 {
                board_str.push_str("   ├───┼───┼───┼───┼───┼───┼───┼───┤\n");
            }
        }
        board_str.push_str("   └───┴───┴───┴───┴───┴───┴───┴───┘\n");
        board_str.push_str("     A   B   C   D   E   F   G   H  \n");
        write!(f, "{board_str}")
    }
}

impl MyGame {
    pub fn game_loop(&mut self) {
        loop {
            println!("{self}");

            print!("Start square: "); 
            let _  = io::stdout().flush();

            let mut start = String::new();

            io::stdin()
                .read_line(&mut start)
                .expect("IO error");

            print!("End square: ");
            let _  = io::stdout().flush();

            let mut end = String::new();

            io::stdin()
                .read_line(&mut end)
                .expect("IO error");

            let start_square = Square::from_str(&start[..]);
            let end_square = Square::from_str(&end[..]);

            if let Err(_) = start_square {
                println!("Invalid");
                continue;
            }

            if let Err(_) = end_square {
                println!("Invalid");
                continue;
            }

            let start_square = start_square.unwrap();
            let end_square = end_square.unwrap();

            if let None = self.0.current_position().piece_on(start_square) {
                println!("Must be piece on start");
                continue;
            }

            let promotion = if self.0.current_position().piece_on(start_square).unwrap() == Piece::Pawn && end_square.get_rank() == Rank::Eighth || end_square.get_rank() == Rank::First {
                let mut piece = String::new();

                io::stdin()
                    .read_line(&mut piece)
                    .expect("IO error");

                Some(piece_from_string(piece))
            } else {
                None
            };

            self.0.make_move(ChessMove::new(start_square, end_square, promotion));
            if let None = self.0.result() {
                continue;
            } else {
                println!("{self}");
                match self.0.result().unwrap() {
                    GameResult::WhiteResigns |
                    GameResult::BlackCheckmates => { 
                        println!("Black Wins!");
                        break;
                    },
                    GameResult::BlackResigns |
                    GameResult::WhiteCheckmates => {
                        println!("White Wins!");
                        break;
                    },
                    GameResult::Stalemate |
                    GameResult::DrawAccepted |
                    GameResult::DrawDeclared => {
                        println!("Nobody wins");
                        break;
                    }
                }
            }
        }
    }
}

fn piece_from_string(string: String) -> Piece {
    match &string[..] {
        "pawn\n"   => Piece::Pawn,
        "king\n"   => Piece::King,
        "queen\n"  => Piece::Queen,
        "bishop\n" => Piece::Bishop,
        "knight\n" => Piece::Knight,
        "rook\n"   => Piece::Rook,
        // Should never happen
        _        => Piece::Pawn,
    }
}
