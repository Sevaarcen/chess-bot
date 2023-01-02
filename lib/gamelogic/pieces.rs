use std::cmp::min;

use super::{board::ChessBoard, ChessMove, MoveType, ChessError, index_pair_to_name};



#[derive(Copy, Clone, Debug)]
pub struct ChessPiece {
    pub position: (usize, usize),  // col, row (e.g. 0,0 = a1, 7,7 = h8)
    pub side: Side,
    pub piece_type: PieceType,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Side {
    White,
    Black
}


impl std::ops::Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White
        }
    }
}


impl ChessPiece {
    pub fn get_material(self: &Self) -> usize {
        match self.piece_type {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Queen => 9,
            PieceType::King => 42, // TODO maybe re-examine if there's a better way to handle this given a King can never be captured.
        }
    }

    /// For a given piece, get a list of all possible moves the piece could make
    pub fn get_moves(self: &Self, board: &ChessBoard) -> Vec<ChessMove> {
        let unfiltered_moves = match self.piece_type {
            PieceType::Pawn => {
                get_pawn_moves(self, board)
            },
            PieceType::Rook => {
                get_rook_moves(self, board)
            },
            PieceType::Knight => {
                get_knight_moves(self, board)
            },
            PieceType::Bishop => {
                get_bishop_moves(self, board)
            },
            PieceType::Queen => {
                get_queen_moves(self, board)
            },
            PieceType::King => {
                get_king_moves(self, board)
            },
        };

        // for all the possible moves, remove any where actually performing said move would result in the king being threatened
        unfiltered_moves.into_iter()
            .filter(|m| {
                !move_would_cause_self_check(board, m)
            })
            .collect()
    }

    pub fn get_specific_move(self: &Self, board: &ChessBoard, desired_move: (usize, usize)) -> Result<ChessMove, ChessError> {
        let valid_moves = self.get_moves(board);

        match valid_moves.into_iter().find(|m| m.destination == desired_move) {
            Some(m) => {
                Ok(m)
            },
            None => {
                Err(ChessError::InvalidMove(format!("'{}' is not a valid move for piece: {:?}", index_pair_to_name(desired_move.0, desired_move.1)?, self)))
            }
        }
    }

    pub fn get_threats(self: &Self, board: &ChessBoard) -> Vec<(usize, usize)> {
        match self.piece_type {
            PieceType::Pawn => {
                get_pawn_threats(self, board)
            },
            PieceType::Rook => {
                get_rook_threats(self, board)
            },
            PieceType::Knight => {
                get_knight_threats(self, board)
            },
            PieceType::Bishop => {
                get_bishop_threats(self, board)
            },
            PieceType::Queen => {
                get_queen_threats(self, board)
            },
            PieceType::King => {
                get_king_threats(self, board)
            },
        }
    }
}


fn move_would_cause_self_check(board: &ChessBoard, the_move: &ChessMove) -> bool {
    // create a copy of the current board state where we can perform the move and then check the result.
    let mut board_copy = board.clone();
    board_copy.perform_move(the_move).unwrap();

    // check if the King is in check for the side that just moved
    let piece = board.get_square_by_index(the_move.from_square.0, the_move.from_square.1).expect(format!("Tried to get a piece at position {:?} but piece didn't exist", the_move.from_square).as_str());
    
    board_copy.is_checked(piece.side)
}


fn get_pawn_moves(piece: &ChessPiece, board: &ChessBoard) -> Vec<ChessMove> {
    let mut possible_moves = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;
    if piece.side == Side::White {
        // double move only if on starting rank and both the squares ahead are not ocupied
        if current_row == 1 && board.get_square_by_index(current_col, current_row + 1).is_none() && board.get_square_by_index(current_col, current_row + 2).is_none() {
            let destination = (current_col, current_row + 2);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::DoubleAdvance,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // otherwise move forward as long as space is not occupied
        if board.get_square_by_index(current_col, current_row + 1).is_none() {
            let destination = (current_col, current_row + 1);
            let move_type = match destination.1 == 7 {
                true => MoveType::Promotion,
                false => MoveType::Standard
            };
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // check possible captures
        // negative side capture -- not at edge of board and space is occupied by piece of opposing side
        if current_col >= 1 && board.get_square_by_index(current_col - 1, current_row + 1).is_some() && board.get_square_by_index(current_col - 1, current_row + 1).unwrap().side != piece.side {
            let destination = (current_col - 1, current_row + 1);
            let move_type = match destination.1 == 7 {
                true => MoveType::Promotion,
                false => MoveType::Standard
            };
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type,
                captures: Some(destination),
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // positive side capture -- not at edge of board and space is occupied by piece of opposing side
        if current_col <= 6 && board.get_square_by_index(current_col + 1, current_row + 1).is_some() && board.get_square_by_index(current_col + 1, current_row + 1).unwrap().side != piece.side {
            let destination = (current_col + 1, current_row + 1);
            let move_type = match destination.1 == 7 {
                true => MoveType::Promotion,
                false => MoveType::Standard
            };
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type,
                captures: Some(destination),
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // if in position for en passtant move, add it to the list
        if board.state.en_passant_column.is_some() && current_row == 4 && current_col.abs_diff(board.state.en_passant_column.unwrap()) == 1 {
            let destination = (board.state.en_passant_column.unwrap(), current_row + 1);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::EnPassant,
                captures: Some((destination.0, destination.1 - 1)),
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
    }
    // Black pawn moves
    else {
        // double move only if on starting rank and both the squares ahead are not ocupied
        if current_row == 6 && board.get_square_by_index(current_col, current_row - 1).is_none() && board.get_square_by_index(current_col, current_row - 2).is_none() {
            let destination = (current_col, current_row - 2);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::DoubleAdvance,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // otherwise move forward as long as space is not occupied
        if board.get_square_by_index(current_col, current_row - 1).is_none() {
            let destination = (current_col, current_row - 1);
            let move_type = match destination.1 == 0 {
                true => MoveType::Promotion,
                false => MoveType::Standard
            };
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // check possible captures
        // negative side capture -- not at edge of board and space is occupied by piece of opposing side
        if current_col >= 1 && board.get_square_by_index(current_col - 1, current_row - 1).is_some() && board.get_square_by_index(current_col - 1, current_row - 1).unwrap().side != piece.side {
            let destination = (current_col - 1, current_row - 1);
            let move_type = match destination.1 == 0 {
                true => MoveType::Promotion,
                false => MoveType::Standard
            };
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type,
                captures: Some(destination),
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // positive side capture -- not at edge of board and space is occupied by piece of opposing side
        if current_col <= 6 && board.get_square_by_index(current_col + 1, current_row - 1).is_some() && board.get_square_by_index(current_col + 1, current_row - 1).unwrap().side != piece.side {
            let destination = (current_col + 1, current_row - 1);
            let move_type = match destination.1 == 0 {
                true => MoveType::Promotion,
                false => MoveType::Standard
            };
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type,
                captures: Some(destination),
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // if in position for en passtant move, add it to the list
        if board.state.en_passant_column.is_some() && current_row == 3 && current_col.abs_diff(board.state.en_passant_column.unwrap()) == 1 {
            let destination = (board.state.en_passant_column.unwrap(), current_row - 1);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::EnPassant,
                captures: Some((destination.0, destination.1 + 1)),
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
    }
    possible_moves
}


fn get_pawn_threats(piece: &ChessPiece, _board: &ChessBoard) -> Vec<(usize, usize)> {
    let mut threatened_squares = Vec::new();    
    let current_col = piece.position.0;
    let current_row = piece.position.1;
    if current_row % 7 == 0 {
        return threatened_squares;
    }
    if piece.side == Side::White {
        if current_col > 0 {
            threatened_squares.push((current_col - 1, current_row + 1));
        }
        if current_col < 7 {
            threatened_squares.push((current_col + 1, current_row + 1));
        }
    } else {
        if current_col > 0 {
            threatened_squares.push((current_col - 1, current_row - 1));
        }
        if current_col < 7 {
            threatened_squares.push((current_col + 1, current_row - 1));
        }
    }
    threatened_squares
}


fn get_rook_moves(piece: &ChessPiece, board: &ChessBoard) -> Vec<ChessMove> {
    let mut possible_moves = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;
    // add all possible squares in current row in the negative direction until end of board or intersection with another game piece
    for col in (0..current_col).rev() {
        if board.get_square_by_index(col, current_row).is_none() {
            let destination = (col, current_row);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        } else {
            if board.get_square_by_index(col, current_row).unwrap().side != piece.side {
                let destination = (col, current_row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: Some(destination),
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
                break
            } else {
                break
            }
        }
    }
    // add all possible squares in current row in the positive direction until end of board or intersection with another game piece
    for col in current_col+1..8 {
        if board.get_square_by_index(col, current_row).is_none() {
            let destination = (col, current_row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: None,
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
        } else {
            if board.get_square_by_index(col, current_row).unwrap().side != piece.side {
                let destination = (col, current_row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: Some(destination),
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
                break
            } else {
                break
            }
        }
    }
    // add all possible squares in current column in the negative direction until end of board or intersection with another game piece
    for row in (0..current_row).rev() {
        if board.get_square_by_index(current_col, row).is_none() {
            let destination = (current_col, row);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        } else {
            if board.get_square_by_index(current_col, row).unwrap().side != piece.side {
                let destination = (current_col, row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: Some(destination),
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
                break
            } else {
                break
            }
        }
    }
    // add all possible squares in current column in the positive direction until end of board or intersection with another game piece
    for row in current_row+1..8 {
        if board.get_square_by_index(current_col, row).is_none() {
            let destination = (current_col, row);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        } else {
            if board.get_square_by_index(current_col, row).unwrap().side != piece.side {
                let destination = (current_col, row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: Some(destination),
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
                break
            } else {
                break
            }
        }
    }
    // TODO handle castling
    possible_moves
}


fn get_rook_threats(piece: &ChessPiece, board: &ChessBoard) -> Vec<(usize, usize)> {
    let mut threatened_squares = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;
    // add all possible squares in current row in the negative direction until end of board or intersection with another game piece
    for col in (0..current_col).rev() {
        if board.get_square_by_index(col, current_row).is_none() {
            threatened_squares.push((col, current_row));
        } else {
            threatened_squares.push((col, current_row));
            break
        }
    }
    // add all possible squares in current row in the positive direction until end of board or intersection with another game piece
    for col in current_col+1..8 {
        if board.get_square_by_index(col, current_row).is_none() {
            threatened_squares.push((col, current_row));
        } else {
            threatened_squares.push((col, current_row));
            break
        }
    }
    // add all possible squares in current column in the negative direction until end of board or intersection with another game piece
    for row in (0..current_row).rev() {
        if board.get_square_by_index(current_col, row).is_none() {
            threatened_squares.push((current_col, row));
        } else {
            threatened_squares.push((current_col, row));
            break
        }
    }
    // add all possible squares in current column in the positive direction until end of board or intersection with a piece of th
    for row in current_row+1..8 {
        if board.get_square_by_index(current_col, row).is_none() {
            threatened_squares.push((current_col, row));
        } else {
            threatened_squares.push((current_col, row));
            break
        }
    }
    threatened_squares
}


fn get_knight_moves(piece: &ChessPiece, board: &ChessBoard) -> Vec<ChessMove> {
    let mut possible_moves = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;

    // double left moves
    if current_col > 1 {
        // 2 left, 1 up
        if current_row < 7 && (board.get_square_by_index(current_col - 2, current_row + 1).is_none() || board.get_square_by_index(current_col - 2, current_row + 1).unwrap().side != piece.side) {
            let destination = (current_col - 2, current_row + 1);
            let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: dest_capture,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // 2 left, 1 down
        if current_row > 0 && (board.get_square_by_index(current_col - 2, current_row - 1).is_none() || board.get_square_by_index(current_col - 2, current_row - 1).unwrap().side != piece.side) {
            let destination = (current_col - 2, current_row - 1);
            let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: dest_capture,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
    }
    // double right moves
    if current_col < 6 {
        // 2 right, 1 up
        if current_row < 7 && (board.get_square_by_index(current_col + 2, current_row + 1).is_none() || board.get_square_by_index(current_col + 2, current_row + 1).unwrap().side != piece.side) {
            let destination = (current_col + 2, current_row + 1);
            let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: dest_capture,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // 2 left, 1 down
        if current_row > 0 && (board.get_square_by_index(current_col + 2, current_row - 1).is_none() || board.get_square_by_index(current_col + 2, current_row - 1).unwrap().side != piece.side) {
            let destination = (current_col + 2, current_row - 1);
            let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: dest_capture,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
    }
    // double up moves
    if current_row < 6 {
        // 2 up, 1 left
        if current_col > 0 && (board.get_square_by_index(current_col - 1, current_row + 2).is_none() || board.get_square_by_index(current_col - 1, current_row + 2).unwrap().side != piece.side) {
            let destination = (current_col - 1, current_row + 2);
            let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: dest_capture,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // 2 up, 1 right
        if current_col < 7 && (board.get_square_by_index(current_col + 1, current_row + 2).is_none() || board.get_square_by_index(current_col + 1, current_row + 2).unwrap().side != piece.side) {
            let destination = (current_col + 1, current_row + 2);
            let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: dest_capture,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
    }
    // double down moves
    if current_row > 1 {
        // 2 down, 1 left
        if current_col > 0 && (board.get_square_by_index(current_col - 1, current_row - 2).is_none() || board.get_square_by_index(current_col - 1, current_row - 2).unwrap().side != piece.side) {
            let destination = (current_col - 1, current_row - 2);
            let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: dest_capture,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
        // 2 down, 1 left
        if current_col < 7 && (board.get_square_by_index(current_col + 1, current_row - 2).is_none() || board.get_square_by_index(current_col + 1, current_row - 2).unwrap().side != piece.side) {
            let destination = (current_col + 1, current_row - 2);
            let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: dest_capture,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        }
    }
    possible_moves
}


fn get_knight_threats(piece: &ChessPiece, _board: &ChessBoard) -> Vec<(usize, usize)> {
    let mut threatened_squares = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;

    // double left moves
    if current_col > 1 {
        // 2 left, 1 up
        if current_row < 7 {
                threatened_squares.push((current_col - 2, current_row + 1));
        }
        // 2 left, 1 down
        if current_row > 0 {
            threatened_squares.push((current_col - 2, current_row - 1));
        }
    }
    // double right moves
    if current_col < 6 {
        // 2 right, 1 up
        if current_row < 7 {
                threatened_squares.push((current_col + 2, current_row + 1));
        }
        // 2 left, 1 down
        if current_row > 0 {
            threatened_squares.push((current_col + 2, current_row - 1));
        }
    }
    // double up moves
    if current_row < 6 {
        // 2 up, 1 left
        if current_col > 0 {
                threatened_squares.push((current_col - 1, current_row + 2));
        }
        // 2 up, 1 right
        if current_col < 7 {
            threatened_squares.push((current_col + 1, current_row + 2));
        }
    }
    // double down moves
    if current_row > 1 {
        // 2 down, 1 left
        if current_col > 0 {
                threatened_squares.push((current_col - 1, current_row - 2));
        }
        // 2 down, 1 left
        if current_col < 7 {
            threatened_squares.push((current_col + 1, current_row - 2));
        }
    }

    threatened_squares
}


fn get_bishop_moves(piece: &ChessPiece, board: &ChessBoard) -> Vec<ChessMove> {
    let mut possible_moves = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;

    // diagonal moves from current column/row up and to left
    let up_left_count = min(current_col, 7 - current_row);
    for diag_up_left in 1..=up_left_count {
        let new_col = current_col - diag_up_left;
        let new_row = current_row + diag_up_left;
        if new_col > 7 || new_row > 7 {
            break;
        }
        if board.get_square_by_index(new_col, new_row).is_none() {
            let destination = (new_col, new_row);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        } else {
            if board.get_square_by_index(new_col, new_row).unwrap().side != piece.side {
                let destination = (new_col, new_row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: Some(destination),
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
                break;
            } else {
                break;
            }
        }
    }
    // diagonal moves from current column/row up and to right
    let up_right_count = min(7 - current_col, 7 - current_row);
    for diag_up_right in 1..=up_right_count {
        let new_col = current_col + diag_up_right;
        let new_row = current_row + diag_up_right;
        if new_col > 7 || new_row > 7 {
            break;
        }
        if board.get_square_by_index(new_col, new_row).is_none() {
            let destination = (new_col, new_row);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        } else {
            if board.get_square_by_index(new_col, new_row).unwrap().side != piece.side {
                let destination = (new_col, new_row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: Some(destination),
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
                break;
            } else {
                break;
            }
        }
    }
    // diagonal moves from current column/row down and to left
    let down_left_count = min(current_col, current_row);
    for diag_down_left in 1..=down_left_count {
        let new_col = current_col - diag_down_left;
        let new_row = current_row - diag_down_left;
        if new_col > 7 || new_row > 7 {
            break;
        }
        if board.get_square_by_index(new_col, new_row).is_none() {
            let destination = (new_col, new_row);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        } else {
            if board.get_square_by_index(new_col, new_row).unwrap().side != piece.side {
                let destination = (new_col, new_row);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: Some(destination),
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
                break;
            } else {
                break;
            }
        }
    }
    // diagonal moves from current column/row down and to right
    let down_right_count = min(7 - current_col, current_row);
    for diag_down_right in 1..=down_right_count {
        let new_col = current_col + diag_down_right;
        let new_row = current_row - diag_down_right;
        if new_col > 7 || new_row > 7 {
            break;
        }
        if board.get_square_by_index(new_col, new_row).is_none() {
            let destination = (new_col, new_row);
            possible_moves.push(ChessMove {
                from_square: (current_col, current_row),
                destination,
                move_type: MoveType::Standard,
                captures: None,
                dest_threatened: board.is_square_threatened(!piece.side, destination),
                dest_defended: board.is_square_threatened(piece.side, destination),
            });
        } else {
            if board.get_square_by_index(new_col, new_row).unwrap().side != piece.side {
                let destination = (new_col, new_row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: Some(destination),
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
                break;
            } else {
                break;
            }
        }
    }
    possible_moves
}


fn get_bishop_threats(piece: &ChessPiece, board: &ChessBoard) -> Vec<(usize, usize)> {
    let mut threatened_squares = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;

    // diagonal moves from current column/row up and to left
    let up_left_count = min(current_col, 7 - current_row);
    for diag_up_left in 1..=up_left_count {
        let new_col = current_col - diag_up_left;
        let new_row = current_row + diag_up_left;
        if new_col > 7 || new_row > 7 {
            break;
        }
        threatened_squares.push((new_col, new_row));
        if board.get_square_by_index(new_col, new_row).is_some() {
            break;
        }
    }
    // diagonal moves from current column/row up and to right
    let up_right_count = min(7 - current_col, 7 - current_row);
    for diag_up_right in 1..=up_right_count {
        let new_col = current_col + diag_up_right;
        let new_row = current_row + diag_up_right;
        if new_col > 7 || new_row > 7 {
            break;
        }
        threatened_squares.push((new_col, new_row));
        if board.get_square_by_index(new_col, new_row).is_some() {
            break;
        }
    }
    // diagonal moves from current column/row down and to left
    let down_left_count = min(current_col, current_row);
    for diag_down_left in 1..=down_left_count {
        let new_col = current_col - diag_down_left;
        let new_row = current_row - diag_down_left;
        if new_col > 7 || new_row > 7 {
            break;
        }
        threatened_squares.push((new_col, new_row));
        if board.get_square_by_index(new_col, new_row).is_some() {
            break;
        }
    }
    // diagonal moves from current column/row down and to right
    let down_right_count = min(7 - current_col, current_row);
    for diag_down_right in 1..=down_right_count {
        let new_col = current_col + diag_down_right;
        let new_row = current_row - diag_down_right;
        if new_col > 7 || new_row > 7 {
            break;
        }
        threatened_squares.push((new_col, new_row));
        if board.get_square_by_index(new_col, new_row).is_some() {
            break;
        }
    }

    threatened_squares
}


fn get_queen_moves(piece: &ChessPiece, board: &ChessBoard) -> Vec<ChessMove> {
    let mut possible_moves = Vec::new();
    let mut diag_moves = get_bishop_moves(piece, board);
    possible_moves.append(&mut diag_moves);
    let mut hori_moves = get_rook_moves(piece, board);
    possible_moves.append(&mut hori_moves);
    possible_moves
}


fn get_queen_threats(piece: &ChessPiece, board: &ChessBoard) -> Vec<(usize, usize)> {
    let mut threatened_squares = Vec::new();
    let mut diag_threats = get_bishop_threats(piece, board);
    threatened_squares.append(&mut diag_threats);
    let mut hori_threats = get_rook_threats(piece, board);
    threatened_squares.append(&mut hori_threats);
    threatened_squares
}


fn get_king_moves(piece: &ChessPiece, board: &ChessBoard) -> Vec<ChessMove> {
    let mut possible_moves = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;
    // iterate over [0, 1, 2]
    for col_shift in 0..3 {
        for row_shift in 0..3 {
            // ignore any moves which would move off the board
            if (col_shift == 0 && current_col == 0) || (row_shift == 0 && current_row == 0) || (col_shift == 2 && current_col == 7) || (row_shift == 2 && current_row == 7) {
                continue;
            }

            // ignore the center shift, as it's the current square
            if col_shift == 1 && row_shift == 1 {
                continue;
            }

            let new_col = current_col + col_shift - 1;
            let new_row = current_row + row_shift - 1;

            if board.get_square_by_index(new_col, new_row).is_none() {
                let destination = (new_col, new_row);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: None,
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
            } else if board.get_square_by_index(new_col, new_row).unwrap().side != piece.side {
                let destination = (new_col, new_row);
                let dest_capture = board.get_square_by_index(destination.0, destination.1).map(|p| p.position);
                possible_moves.push(ChessMove {
                    from_square: (current_col, current_row),
                    destination,
                    move_type: MoveType::Standard,
                    captures: dest_capture,
                    dest_threatened: board.is_square_threatened(!piece.side, destination),
                    dest_defended: board.is_square_threatened(piece.side, destination),
                });
            }
        }
    }
    // add castling moves
    match piece.side {
        Side::White => {
            if !board.state.white_king_moved && !board.is_checked(Side::White) {
                if !board.state.white_queen_rook_moved {
                    // check there's no pieces in the way AND the move wouldn't be a check
                    let mut can_castle = true;
                    for col in (1..current_col).rev() {
                        if board.get_square_by_index(col, current_row).is_some() {
                            can_castle = false;
                            break
                        }
                        // Calculate intermediate moves and verify if the king were to take the move it wouldn't result in a check
                        // b/c piece controls what moves are possible, the board allows us to jump king multiple spaces for testing :)
                        let intermediate_move = ChessMove { 
                            from_square: (current_col, current_row),  
                            destination: (col, current_row),
                            move_type: MoveType::Standard,
                            captures: None,
                            dest_threatened: false,
                            dest_defended: false
                        };
                        if move_would_cause_self_check(board, &intermediate_move) {
                            can_castle = false;
                            break
                        }
                    };
                    // verify the piece in the castle's position is actually the castle of the correct side, and wasn't captured at some point.
                    if can_castle && board.get_square_by_index(1, current_row).is_some() {
                        let rook_piece = board.get_square_by_index(1, current_row).unwrap();
                        can_castle = piece.side == rook_piece.side && rook_piece.piece_type == PieceType::Rook;
                    }
                    // if all checks pass, we can castle
                    if can_castle {
                        possible_moves.push(ChessMove {
                            from_square: (current_col, current_row),
                            destination: (1, current_row),
                            move_type: MoveType::Castle,
                            captures: None,
                            dest_threatened: false,  // move must never result in the king being threatened
                            dest_defended: true,  // b/c rook is always at king's side afterwards, the 
                        });
                    }
                }
                if !board.state.white_king_rook_moved {
                    // check there's no pieces in the way
                    let mut can_castle = true;
                    for col in current_col+1..7 {
                        if board.get_square_by_index(col, current_row).is_some() {
                            can_castle = false;
                            break
                        }
                        // Calculate intermediate moves and verify if the king were to take the move it wouldn't result in a check
                        // b/c piece controls what moves are possible, the board allows us to jump king multiple spaces for testing :)
                        let intermediate_move = ChessMove { 
                            from_square: (current_col, current_row),  
                            destination: (col, current_row),
                            move_type: MoveType::Standard,
                            captures: None,
                            dest_threatened: false,
                            dest_defended: false
                        };
                        if move_would_cause_self_check(board, &intermediate_move) {
                            can_castle = false;
                            break
                        }
                    }
                    // verify the piece in the castle's position is actually the castle of the correct side, and wasn't captured at some point.
                    if can_castle && board.get_square_by_index(7, current_row).is_some() {
                        let rook_piece = board.get_square_by_index(1, current_row).unwrap();
                        can_castle = piece.side == rook_piece.side && rook_piece.piece_type == PieceType::Rook;
                    }
                    if can_castle {
                        possible_moves.push(ChessMove {
                            from_square: (current_col, current_row),
                            destination: (6, current_row),
                            move_type: MoveType::Castle,
                            captures: None,
                            dest_threatened: false,  // move must never result in the king being threatened
                            dest_defended: true,  // b/c rook is always at king's side afterwards, the 
                        });
                    }
                }
            }
        },
        Side::Black => {
            if !board.state.black_king_moved && !board.is_checked(Side::Black) {
                if !board.state.black_queen_rook_moved {
                    // check there's no pieces in the way
                    let mut can_castle = true;
                    for col in (1..current_col).rev() {
                        if board.get_square_by_index(col, current_row).is_some() {
                            can_castle = false;
                            break
                        }
                        // Calculate intermediate moves and verify if the king were to take the move it wouldn't result in a check
                        // b/c piece controls what moves are possible, the board allows us to jump king multiple spaces for testing :)
                        let intermediate_move = ChessMove { 
                            from_square: (current_col, current_row),  
                            destination: (col, current_row),
                            move_type: MoveType::Standard,
                            captures: None,
                            dest_threatened: false,
                            dest_defended: false
                        };
                        if move_would_cause_self_check(board, &intermediate_move) {
                            can_castle = false;
                            break
                        }
                    }
                    // verify the piece in the castle's position is actually the castle of the correct side, and wasn't captured at some point.
                    if can_castle && board.get_square_by_index(1, current_row).is_some() {
                        let rook_piece = board.get_square_by_index(1, current_row).unwrap();
                        can_castle = piece.side == rook_piece.side && rook_piece.piece_type == PieceType::Rook;
                    }
                    if can_castle {
                        possible_moves.push(ChessMove {
                            from_square: (current_col, current_row),
                            destination: (1, current_row),
                            move_type: MoveType::Castle,
                            captures: None,
                            dest_threatened: false,  // move must never result in the king being threatened
                            dest_defended: true,  // b/c rook is always at king's side afterwards, the 
                        });
                    }
                }
                if !board.state.black_king_rook_moved {
                    // check there's no pieces in the way
                    let mut can_castle = true;
                    for col in current_col+1..7 {
                        if board.get_square_by_index(col, current_row).is_some() {
                            can_castle = false;
                            break
                        }
                        // Calculate intermediate moves and verify if the king were to take the move it wouldn't result in a check
                        // b/c piece controls what moves are possible, the board allows us to jump king multiple spaces for testing :)
                        let intermediate_move = ChessMove { 
                            from_square: (current_col, current_row),  
                            destination: (col, current_row),
                            move_type: MoveType::Standard,
                            captures: None,
                            dest_threatened: false,
                            dest_defended: false
                        };
                        if move_would_cause_self_check(board, &intermediate_move) {
                            can_castle = false;
                            break
                        }
                    }
                    // verify the piece in the castle's position is actually the castle of the correct side, and wasn't captured at some point.
                    if can_castle && board.get_square_by_index(7, current_row).is_some() {
                        let rook_piece = board.get_square_by_index(1, current_row).unwrap();
                        can_castle = piece.side == rook_piece.side && rook_piece.piece_type == PieceType::Rook;
                    }
                    if can_castle {
                        possible_moves.push(ChessMove {
                            from_square: (current_col, current_row),
                            destination: (6, current_row),
                            move_type: MoveType::Castle,
                            captures: None,
                            dest_threatened: false,  // move must never result in the king being threatened
                            dest_defended: true,  // b/c rook is always at king's side afterwards, the 
                        });
                    }
                }
            }
        },
    }

    possible_moves
}


fn get_king_threats(piece: &ChessPiece, _board: &ChessBoard) -> Vec<(usize, usize)> {
    let mut threatened_squares = Vec::new();
    let current_col = piece.position.0;
    let current_row = piece.position.1;
    for col_shift in 0..3 {
        for row_shift in 0..3 {
            // ignore any moves which would move off the board
            if (col_shift == 0 && current_col == 0) || (row_shift == 0 && current_row == 0) || (col_shift == 2 && current_col == 7) || (row_shift == 2 && current_row == 7) {
                continue;
            }

            // ignore the center shift, as it's the current square
            if col_shift == 1 && row_shift == 1 {
                continue;
            }

            let new_col = current_col + col_shift - 1;
            let new_row = current_row + row_shift - 1;

            threatened_squares.push((new_col, new_row));
        }
    }
    threatened_squares
}