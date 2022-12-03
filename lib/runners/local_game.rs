use crate::{gamelogic::{pieces::Side, board::ChessBoard, index_pair_to_name, GameEnd}, stratagems::Stratagem};

use super::{Connector, ConnectorError};

use std::io::{stdin, stdout, Write};

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
        println!("Current Board State\n{}", self.board);
        let user_move = 'outer: loop {
            let piece = loop {
                print!("Enter a valid square for the game piece: ");
                let _ = stdout().flush();
                let mut s=String::new();
                stdin().read_line(&mut s).unwrap();
                s = s.trim().to_string();
                let piece_res = self.board.get_square_by_name(s);
                match piece_res {
                    Ok(square) => match square {
                        Some(p) => {
                            if p.side != self.side {
                                println!("That piece doesn't belong to your side...");
                                continue 'outer;
                            }
                            break p
                        },  // the user put in a valid piece, return it out of just the loop
                        None => println!("There is no piece on that square")
                    }
                    Err(e) => println!("Invalid square name: {}", e)
                }
            };
            let mut all_piece_moves = piece.get_moves(&self.board);
            if all_piece_moves.is_empty() {
                println!("You can't move that piece -- there are no valid moves");
                continue 'outer;
            }
            let valid_move_names = all_piece_moves.iter().enumerate().map(|(index, m)| (index, index_pair_to_name(m.destination.0, m.destination.1).unwrap())).collect::<Vec<(usize, String)>>();
            let just_move_names = valid_move_names.iter().map(|(_, n)| n).collect::<Vec<&String>>();
            let chosen_move = loop {
                println!("Valid Moves are: {:?}", just_move_names);
                print!("Enter your move (or 'exit'): ");
                let _ = stdout().flush();
                let mut s=String::new();
                stdin().read_line(&mut s).unwrap();
                s = s.trim().to_string();
                if s == "exit" {
                    continue 'outer;
                }
                if !just_move_names.contains(&&s) {
                    println!("That's not one of the valid moves... come on dude.");
                    continue;
                }
                break s;
            };
            let chosen_index = valid_move_names.iter().find(|(_, m)| m == &chosen_move).unwrap().0;
            break all_piece_moves.remove(chosen_index)
        };

        // perform the move the user requested
        self.board.perform_move(&user_move).expect("Could not perform player move");
        println!("Board After Player Move:\n{}", self.board);
        // get the bot move and perform it too
        Ok(())
    }

    fn execute_bot_move(self: &mut Self) -> Result<(), ConnectorError> {
        let bot_move = self.bot_opponent.get_move(&self.board);
        println!("Bot chose move: {:#?}", bot_move);
        self.board.perform_move(&bot_move).expect("Could not perform bot move");
        println!("Board After Bot Move:\n{}\n\n", self.board);
        Ok(()) // the game is entirely managed by the internal board state, no external system needs to be interacted with
    }

    fn check_victory(self: &Self) -> Option<GameEnd> {
        self.board.is_game_over()
    }
}