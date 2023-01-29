use crate::gamelogic::{board::ChessBoard, ChessMove, Side};

pub mod random_aggro;
pub mod cole_miner;

pub trait Stratagem {
    fn initialize(side: Side) -> Self where Self: Sized;
    fn get_move(self: &mut Self, board_state: &ChessBoard) -> ChessMove;
}