use rand::Rng;

use crate::gamelogic::{board::ChessBoard, pieces::Side, ChessMove};

use super::Stratagem;

pub struct RandomAggro {
    player_side: Side
}

impl Stratagem for RandomAggro {
    fn initialize(side: Side) -> Self {
        println!("Random Aggressive Strategem is active");
        RandomAggro { player_side: side }
    }

    fn get_move(self: &Self, board_state: &ChessBoard) -> ChessMove {
        let mut possible_moves = board_state.get_all_moves(self.player_side);
        let random_index = rand::thread_rng().gen_range(0..possible_moves.len());
        
        let highest_value_capture = possible_moves.iter()
            .enumerate()
            .filter(
                |(_, m)| m.captures.is_some()
            )
            .max_by_key(
                |(_, m)| {
                    let cap_position = m.captures.unwrap();
                    let capture_piece = board_state.get_square_by_index(cap_position.0, cap_position.1).unwrap();
                    capture_piece.get_material()
                }
            );
        match highest_value_capture {
            Some((index, _)) => possible_moves.remove(index),
            None => possible_moves.remove(random_index)
        }
    }
}