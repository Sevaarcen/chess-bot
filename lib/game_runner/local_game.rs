use crate::{gamelogic::{pieces::Side, board::ChessBoard, ChessMove, index_pair_to_name, name_to_index_pair}, stratagems::Stratagem};

use super::{Connector, ConnectorError};

use std::io::stdin;

pub struct LocalGame {
    board: ChessBoard,
    side: Side,
    bot_opponent: Box<dyn Stratagem>,
}

impl Connector for LocalGame {
    fn initialize(strategem: Box<dyn Stratagem>) -> Result<Self, ConnectorError>  where Self: Sized {
        Ok(LocalGame { 
            board: ChessBoard::new(),
            side: Side::White, // player will always be White because that's easier for me to handle :)
            bot_opponent: strategem
        })
    }

    fn refresh_state(self: &mut Self) -> Result<(), ConnectorError> {
        println!("{}", self.board);
        let user_move = 'outer: loop {
            print!("Enter a valid square for the game piece: ");
            let piece = loop {
                let mut s=String::new();
                stdin().read_line(&mut s).unwrap();
                let piece_res = self.board.get_square_by_name(s);
                match piece_res {
                    Ok(square) => match square {
                        Some(p) => {
                            if p.side != self.side {
                                println!("That piece doesn't belong to your side...");
                                continue;
                            }
                            break p
                        },  // the user put in a valid piece, return it out of just the loop
                        None => println!("There is no piece on that square")
                    }
                    Err(e) => println!("Invalid square name: {}", e)
                }
            };
            let mut all_piece_moves = piece.get_moves(&self.board);
            let valid_move_names = all_piece_moves.iter().enumerate().map(|(index, m)| (index, index_pair_to_name(m.destination.0, m.destination.1).unwrap())).collect::<Vec<(usize, String)>>();
            let just_move_names = valid_move_names.iter().map(|(_, n)| n).collect::<Vec<&String>>();
            println!("Valid Moves are: {:?}", just_move_names);
            print!("Enter your move (or 'restart'): ");
            let chosen_move = loop {
                let mut s=String::new();
                stdin().read_line(&mut s).unwrap();
                if s == "restart" {
                    continue 'outer;
                }
                if !just_move_names.contains(&&s) {
                    println!("That's not one of the valid moves... come on dude.");
                    continue;
                }
                break s;
            };
            let chosen_index = valid_move_names.iter().find(|(index, m)| m == &chosen_move).unwrap().0;
            break all_piece_moves.remove(chosen_index)
        };

        // perform the move the user requested
        self.board.perform_move(&user_move).expect("Could not perform player move");
        // get the bot move and perform it too
        let bot_move = self.bot_opponent.get_move(&self.board);
        println!("Bot chose move: {:#?}", bot_move);
        self.board.perform_move(&bot_move).expect("Could not perform bot move");
        Ok(())
    }

    fn input_move(self: &Self, input: ChessMove) -> Result<(), ConnectorError> {
        Ok(()) // the game is entirely managed by the internal board state, no external system needs to be interacted with
    }
}