use std::fmt::Display;

use super::ChessError;
use super::name_to_index_pair;
use super::pieces::{ChessPiece, Side, PieceType};

use colored::*;

// TODO flatten data structure and keep everything with board? e.g. WHITE_ROOK, BLACK_ROOK?


/// Example Initial Setup: https://www.regencychess.co.uk/images/how-to-set-up-a-chessboard/how-to-set-up-a-chessboard-7.jpg
#[derive(Copy, Clone, Debug)]
pub struct ChessBoard {
    pub squares: [[Option<ChessPiece>; 8]; 8], // 0,0 = a1, 7,7 = h8
    pub state: BoardStateFlags
}

#[derive(Copy, Clone, Debug, Default)]
pub struct BoardStateFlags {
    white_king_moved: bool,
    white_queen_rook_moved: bool,
    white_king_rook_moved: bool,
    black_king_moved: bool,
    black_queen_rook_moved: bool,
    black_king_rook_moved: bool,
    passante_row: Option<usize>,
}


impl ChessBoard {
    pub fn new() -> Self {
        // start with an empty board
        let mut squares = [[None; 8]; 8];

        // setup white main pieces
        squares[0][0] = Some(ChessPiece { position: (0,0), side: Side::White, piece_type: PieceType::Rook});
        squares[1][0] = Some(ChessPiece { position: (1,0), side: Side::White, piece_type: PieceType::Knight});
        squares[2][0] = Some(ChessPiece { position: (2,0), side: Side::White, piece_type: PieceType::Bishop});
        squares[3][0] = Some(ChessPiece { position: (3,0), side: Side::White, piece_type: PieceType::Queen});
        squares[4][0] = Some(ChessPiece { position: (4,0), side: Side::White, piece_type: PieceType::King});
        squares[5][0] = Some(ChessPiece { position: (5,0), side: Side::White, piece_type: PieceType::Bishop});
        squares[6][0] = Some(ChessPiece { position: (6,0), side: Side::White, piece_type: PieceType::Knight});
        squares[7][0] = Some(ChessPiece { position: (7,0), side: Side::White, piece_type: PieceType::Rook});
        // setup black main pieces
        squares[0][7] = Some(ChessPiece { position: (0,7), side: Side::Black, piece_type: PieceType::Rook});
        squares[1][7] = Some(ChessPiece { position: (1,7), side: Side::Black, piece_type: PieceType::Knight});
        squares[2][7] = Some(ChessPiece { position: (2,7), side: Side::Black, piece_type: PieceType::Bishop});
        squares[3][7] = Some(ChessPiece { position: (3,7), side: Side::Black, piece_type: PieceType::Queen});
        squares[4][7] = Some(ChessPiece { position: (4,7), side: Side::Black, piece_type: PieceType::King});
        squares[5][7] = Some(ChessPiece { position: (5,7), side: Side::Black, piece_type: PieceType::Bishop});
        squares[6][7] = Some(ChessPiece { position: (6,7), side: Side::Black, piece_type: PieceType::Knight});
        squares[7][7] = Some(ChessPiece { position: (7,7), side: Side::Black, piece_type: PieceType::Rook});

        // setup pawn rows for both white and black
        for col in 0..8 {
            squares[col][1] = Some(ChessPiece { position: (col,1), side: Side::White, piece_type: PieceType::Pawn});
            squares[col][6] = Some(ChessPiece { position: (col,6), side: Side::Black, piece_type: PieceType::Pawn});
        }

        // create initialized ChessBoard object and pass back to caller
        ChessBoard {
            squares,  // 2d array of columns and rows
            state: BoardStateFlags { ..Default::default() }  // start with all flags false
        }
    }
    
    pub fn get_square_by_index(self: &Self, column: usize, row: usize) -> Option<ChessPiece> {
        // TODO change to result in case given indexes are known to be out of range? or just deal w/ potential run-time error
        self.squares[column][row]
    }

    pub fn get_square_by_name(self: &Self, square_name: String) -> Result<Option<ChessPiece>, ChessError> {
        let (column_index, row_index) = name_to_index_pair(square_name)?;

        Ok(self.get_square_by_index(column_index, row_index))
    }
}


// TODO make a pretty print function that can support an overlay to make things like movement maps or threat maps
impl Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print rows in reverse since the numbers increase from bottom to top
        for row_indx in (0..8).rev() {
            write!(f, "{} ", format!("{}", row_indx+1).black())?;
            for col_indx in 0..8 {
                let char = match self.squares[col_indx][row_indx] {
                    Some(piece) => {
                        let label = match piece.piece_type {
                            PieceType::Pawn => 'p',
                            PieceType::Rook => 'R',
                            PieceType::Knight => 'N',
                            PieceType::Bishop => 'B',
                            PieceType::Queen => 'Q',
                            PieceType::King => 'K',
                        };
                        match piece.side {
                            Side::White => label.to_string().white(),
                            Side::Black => label.to_string().blue(),
                        }
                    },
                    None => '·'.to_string().black()
                };
                write!(f, "{}", char)?;
            }
            write!(f, "\n")?;
        };
        write!(f, "  {}\n", "abcdefgh".black())?;
        Ok(())
    }
}