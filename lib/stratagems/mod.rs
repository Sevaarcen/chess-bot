use crate::gamelogic::{board::ChessBoard, ChessMove, pieces::Side};

pub mod random_aggro;

pub trait Stratagem {
    fn initialize(side: Side) -> Self where Self: Sized;
    fn get_move(self: &Self, board_state: &ChessBoard) -> ChessMove;
}