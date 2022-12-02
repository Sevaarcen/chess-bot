use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Display;
use std::rc::Rc;

use super::ChessError;
use super::ChessMove;
use super::GameEnd;
use super::MoveType;
use super::name_to_index_pair;
use super::pieces::{ChessPiece, Side, PieceType};

use colored::*;


#[derive(Clone, Debug)]
pub struct ChessBoard {
    pub squares: [[Option<ChessPiece>; 8]; 8], // 0,0 = a1, 7,7 = h8
    pub state: BoardStateFlags
}

#[derive(Copy, Clone, Debug, Default)]
pub struct BoardStateFlags {
    pub white_king_moved: bool,
    pub white_queen_rook_moved: bool,
    pub white_king_rook_moved: bool,
    pub black_king_moved: bool,
    pub black_queen_rook_moved: bool,
    pub black_king_rook_moved: bool,
    pub en_passant_column: Option<usize>,
}


impl ChessBoard {
    pub fn new() -> Self {
        // An Image showing a nice view of the Initial Setup: https://www.regencychess.co.uk/images/how-to-set-up-a-chessboard/how-to-set-up-a-chessboard-7.jpg
        // start with an empty board
        let mut squares: [[Option<ChessPiece>; 8]; 8] = Default::default();

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

    pub fn perform_move(self: &mut Self, chess_move: &ChessMove) -> Result<(), ()> {
        let current_position = chess_move.from_square;
        let mut piece = self.get_square_by_index(current_position.0, current_position.1).unwrap();
        let dest_col = chess_move.destination.0;
        let dest_row = chess_move.destination.1;

        // move piece from current position to destination
        piece.position = chess_move.destination;
        self.squares[current_position.0][current_position.1] = None;
        self.squares[dest_col][dest_row] = Some(piece);

        // handle flags depending on if special move is performed
        match chess_move.move_type {
            MoveType::EnPassant => {
                let captured = match piece.side {
                    Side::White => self.get_square_by_index(dest_col, dest_row - 1).unwrap(),
                    Side::Black => self.get_square_by_index(dest_col, dest_row + 1).unwrap(),
                };
                self.squares[captured.position.0][captured.position.1] = None;
            },
            MoveType::DoubleAdvance => {
                self.state.en_passant_column = Some(dest_col);
            }
            _ => {
                self.state.en_passant_column = None;
            }
        }

        Ok(())
    }

    pub fn get_threatened(self: &Self, side: Side) -> Vec<(usize, usize)> {
        let mut threatened = Vec::new();
        // for every column and row
        for columns in &self.squares {
            for square in columns {
                // ignore if the square is empty or the piece is not the side of interest
                if square.is_none() || square.unwrap().side != side {
                    continue;
                }
                // generate a list of possible captures and then add to the list.
                let mut piece_threats = square.unwrap().get_threats(&self);
                threatened.append(&mut piece_threats);
            }
        }
        threatened
    }

    pub fn get_threatened_map(self: &Self, side: Side) -> HashSet<(usize, usize)> {
        let threatened_squares_vec = self.get_threatened(side);
        let mut threatened = HashSet::new();
        for square in threatened_squares_vec {
            threatened.insert(square);
        }
        threatened
    }

    pub fn is_square_threatened(self: &Self, side: Side, square: (usize, usize)) -> bool {
        let threatened = self.get_threatened_map(side);
        threatened.contains(&square)
    }

    pub fn is_checked(self: &Self, side: Side) -> bool {
        let king_piece = self.squares.iter()
            .find_map(|row| {
                row.iter()
                    .find(
                        |square| 
                        square.is_some() && (square.unwrap().piece_type == PieceType::King && square.unwrap().side == side)
                    )
                    .map(|s| s.clone()
                )
            })
            .unwrap()
            .unwrap();
        self.is_square_threatened(!side, king_piece.position)
    }

    pub fn get_all_pieces(self: &Self, side: Side) -> Vec<ChessPiece> {
        let mut pieces = Vec::new();
        for columns in self.squares {
            for square in columns {
                // ignore if the square is empty or the piece is not the side of interest
                if square.is_none() || square.unwrap().side != side {
                    continue;
                }
                pieces.push(square.unwrap());
            }
        }
        pieces
    }

    pub fn get_all_moves(self: &Self, side: Side) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for piece in self.get_all_pieces(side) {
            moves.append(&mut piece.get_moves(&self));
        }
        moves
    }

    /// Checks if there's a game ending state for the given board.
    /// 
    /// Reference: https://www.chess.com/article/view/how-chess-games-can-end-8-ways-explained
    pub fn is_game_over(self: &Self) -> Option<GameEnd> {
        let white_is_checked = self.is_checked(Side::White);
        let white_has_no_moves = self.get_all_moves(Side::White).is_empty();
        
        if white_is_checked && white_has_no_moves {
            // Black achieved Checkmate if White remains in Check and has no valid moves remaining to escape
            return Some(GameEnd::BlackVictory);
        } else if white_has_no_moves {
            // If there are no valid moves which White can make, that means the game is in a draw
            return Some(GameEnd::Draw);
        }
        
        let black_is_checked = self.is_checked(Side::Black);
        let black_has_no_moves = self.get_all_moves(Side::Black).is_empty();
        if black_is_checked && black_has_no_moves {
            // White achieved Checkmate if Black remains in Check and has no valid moves remaining to escape
            return Some(GameEnd::WhiteVictory);
        } else if black_has_no_moves {
             // If there are no valid moves which White can make, that means the game is in a draw
            return Some(GameEnd::Draw);
        }

        // Check for insufficient material game ending. This occurs when one side only has a king, or both sides have their king plus a minot piece (bishop or knight)
        let white_pieces = self.get_all_pieces(Side::White);
        let black_pieces = self.get_all_pieces(Side::Black);

        // Game is a draw if both sides are left with only the king
        if white_pieces.len() == 1 && black_pieces.len() == 1 {
            return Some(GameEnd::Draw);
        }
        // Game ends in a draw if White only has their king, ...
        else if white_pieces.len() == 1 {
            // and Black has just the King and a Knight/Bishop
            if black_pieces.len() == 2 && black_pieces.iter().find(|p| p.piece_type != PieceType::King).unwrap().get_material() == 3 {
                return Some(GameEnd::Draw);
            }
            // or has just their King and two Knights
            else if black_pieces.len() == 3 && black_pieces.iter().filter(|p| p.piece_type != PieceType::King).any(|p| p.piece_type != PieceType::Knight) {
                return Some(GameEnd::Draw);
            }
        }
        // Game ends in a draw if Black only has their king, ...
        else if black_pieces.len() == 1 {
            // and Black has just the King and a Knight/Bishop
            if white_pieces.len() == 2 && white_pieces.iter().find(|p| p.piece_type != PieceType::King).unwrap().get_material() == 3 {
                return Some(GameEnd::Draw);
            }
            else if white_pieces.len() == 3 && white_pieces.iter().filter(|p| p.piece_type != PieceType::King).any(|p| p.piece_type != PieceType::Knight) {
                return Some(GameEnd::Draw);
            }
        }
        // Game ends in a Draw if both sides have their Kings and a Knight/Bishop piece each
        else if white_pieces.len() == 2 && black_pieces.len() == 2 && white_pieces.iter().find(|p| p.piece_type != PieceType::King).unwrap().get_material() == 3 && black_pieces.iter().find(|p| p.piece_type != PieceType::King).unwrap().get_material() == 3 {
            return Some(GameEnd::Draw);
        }

        // If no ending state has been identified, the game goes on
        None
    }
}


// TODO make a pretty print function that can support an overlay to make things like movement maps or threat maps
impl Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let white_threat_map = self.get_threatened_map(Side::White);
        let black_threat_map = self.get_threatened_map(Side::Black);
        // print rows in reverse since the numbers increase from bottom to top
        for row_indx in (0..8).rev() {
            write!(f, "{} ", format!("{}", row_indx+1).black())?;
            for col_indx in 0..8 {
                let char = match &self.squares[col_indx][row_indx] {
                    Some(piece) => {
                        match piece.side {
                            Side::White => match piece.piece_type {
                                PieceType::Pawn => "♙ ",
                                PieceType::Rook => "♖ ",
                                PieceType::Knight => "♘ ",
                                PieceType::Bishop => "♗ ",
                                PieceType::Queen => "♕ ",
                                PieceType::King => "♔ ",
                            }.white(),
                            Side::Black => match piece.piece_type {
                                PieceType::Pawn => "♟︎ ",
                                PieceType::Rook => "♜ ",
                                PieceType::Knight => "♞ ",
                                PieceType::Bishop => "♝ ",
                                PieceType::Queen => "♛ ",
                                PieceType::King => "♚ ",
                            }.blue(),
                        }
                    },
                    None => "╶╴".truecolor(128, 128, 128)
                };
                let white_threat = white_threat_map.contains(&(col_indx, row_indx));
                let black_threat = black_threat_map.contains(&(col_indx, row_indx));
                if white_threat && black_threat {
                    write!(f, "{}", char.on_green())?;
                } else if white_threat {
                    write!(f, "{}", char.on_white())?;
                } else if black_threat {
                    write!(f, "{}", char.on_blue())?;
                } else {
                    write!(f, "{}", char)?;
                }
            }
            write!(f, "\n")?;
        };
        write!(f, "  {}\n", "a b c d e f g h".black())?;
        Ok(())
    }
}