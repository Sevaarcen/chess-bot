use crate::{gamelogic::{pieces::Side, board::ChessBoard, index_pair_to_name, GameEnd}, stratagems::Stratagem};

use super::{Runner, RunnerError};

use std::io::{stdin, stdout, Write};

pub struct LocalGame {
    pub board: ChessBoard,
    side: Side,
    bot_opponent: Box<dyn Stratagem>,
    current_turn: Side
}

impl Runner for LocalGame {
    fn initialize<T: Stratagem + 'static>() -> Result<Self, RunnerError>  where Self: Sized {
        let strat = <T as Stratagem>::initialize(Side::Black);
        Ok(LocalGame { 
            board: ChessBoard::new(),
            side: Side::White, // player will always be White because that's easier for me to handle :)
            bot_opponent: Box::new(strat),  // The runner doesn't know, nor care, about the type of the Strategem, as long as the trait is implemented.
            current_turn: Side::White,
        })
    }

    fn refresh_state(self: &mut Self) -> Result<(), RunnerError> {
        println!("Current Board State\n{}", self.board);
        let user_move = 'outer: loop {
            let piece = loop {
                print!("Enter a valid square for the game piece: ");
                let _ = stdout().flush();
                let mut s=String::new();
                stdin().read_line(&mut s).unwrap();
                s = s.trim().to_string();
                if s == "get-state" {
                    println!("{:#?}", self.board.state);
                    continue 'outer;
                }
                let piece_res = self.board.get_square_by_name(s);
                match piece_res {
                    Ok(square) => match square {
                        Some(p) => {
                            if p.side != self.side {
                                let all_piece_moves = p.get_moves(&self.board);
                                let valid_move_names = all_piece_moves.iter().map(|m| index_pair_to_name(m.destination.0, m.destination.1).unwrap()).collect::<Vec<String>>();
                                println!("That piece doesn't belong to your side... but it's valid moves are: {:?}", valid_move_names);
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
        self.board.perform_move_and_record(&user_move).expect("Could not perform player move");
        println!("Board After Player Move:\n{}", self.board);
        // get the bot move and perform it too
        self.current_turn = !self.current_turn;
        Ok(())
    }

    fn execute_bot_move(self: &mut Self) -> Result<(), RunnerError> {
        let bot_move = self.bot_opponent.get_move(&self.board);
        println!("Bot chose move: {:#?}", bot_move);
        self.board.perform_move_and_record(&bot_move).expect("Could not perform bot move");
        self.current_turn = !self.current_turn;
        Ok(()) // the game is entirely managed by the internal board state, no external system needs to be interacted with
    }

    fn check_victory(self: &Self) -> Option<GameEnd> {
        self.board.is_game_over(self.current_turn)
    }
}