use super::board::ChessBoard;

#[derive(Copy, Clone, Debug)]
pub struct ChessPiece {
    pub position: (usize, usize),  // col, row (e.g. 0,0 = a1, 7,7 = h8)
    pub side: Side,
    pub piece_type: PieceType,
}

#[derive(Copy, Clone, Debug)]
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

impl ChessPiece {
    pub fn get_material(self: &Self) -> usize {
        match self.piece_type {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Queen => 9,
            PieceType::King => 0, // TODO figure out how to handle this?
        }
    }

    /// For a given piece, get a list of all possible moves the piece could make
    pub fn get_moves(self: &Self, board: &ChessBoard) -> Vec<(usize, usize)> {
        let current_col = self.position.0;
        let current_row = self.position.1;

        let mut possible_moves = Vec::new();
        match self.piece_type {
            PieceType::Pawn => {
                if self.side == Side::White {
                    // double move only if on starting rank and the square is not ocupied
                    if current_row == 1 && board.get_square_by_index(current_col, current_row + 2).is_none() {
                        possible_moves.push((current_col, current_row + 2));
                    }
                    // otherwise move forward as long as space is not occupied
                    if board.get_square_by_index(current_col, current_row + 1).is_none() {
                        possible_moves.push((current_col, current_row + 1));
                    }
                    // check possible captures
                    // negative side capture -- not at edge of board and space is occupied by piece of opposing side
                    if current_col >= 1 && board.get_square_by_index(current_col - 1, current_row + 1).is_some() && board.get_square_by_index(current_col - 1, current_row + 1).unwrap().side != self.side {
                        possible_moves.push((current_col - 1, current_row + 1));
                    }
                    // positive side capture -- not at edge of board and space is occupied by piece of opposing side
                    if current_col >= 1 && board.get_square_by_index(current_col + 1, current_row + 1).is_some() && board.get_square_by_index(current_col + 1, current_row + 1).unwrap().side != self.side {
                        possible_moves.push((current_col + 1, current_row + 1));
                    }
                } else {
                    // double move only if on starting rank and the square is not ocupied
                    if current_row == 6 && board.get_square_by_index(current_col, current_row - 2).is_none() {
                        possible_moves.push((current_col, current_row - 2));
                    }
                    // otherwise move forward as long as space is not occupied
                    if board.get_square_by_index(current_col, current_row - 1).is_none() {
                        possible_moves.push((current_col, current_row - 1));
                    }
                    // check possible captures
                    // negative side capture -- not at edge of board and space is occupied by piece of opposing side
                    if current_col >= 1 && board.get_square_by_index(current_col - 1, current_row - 1).is_some() && board.get_square_by_index(current_col - 1, current_row - 1).unwrap().side != self.side {
                        possible_moves.push((current_col - 1, current_row - 1));
                    }
                    // positive side capture -- not at edge of board and space is occupied by piece of opposing side
                    if current_col >= 1 && board.get_square_by_index(current_col + 1, current_row - 1).is_some() && board.get_square_by_index(current_col + 1, current_row - 1).unwrap().side != self.side {
                        possible_moves.push((current_col + 1, current_row + 1));
                    }
                }
                // TODO handle passant
            },
            PieceType::Rook => {
                // add all possible squares in current row in the negative direction until end of board or intersection with another game piece
                for col in (0..current_col).rev() {
                    if board.get_square_by_index(col, current_row).is_none() {
                        possible_moves.push((col, current_row));
                    } else {
                        if board.get_square_by_index(col, current_row).unwrap().side != self.side {
                            possible_moves.push((col, current_row));
                            break
                        } else {
                            break
                        }
                    }
                }
                // add all possible squares in current row in the positive direction until end of board or intersection with another game piece
                for col in current_col+1..8 {
                    if board.get_square_by_index(col, current_row).is_none() {
                        possible_moves.push((col, current_row));
                    } else {
                        if board.get_square_by_index(col, current_row).unwrap().side != self.side {
                            possible_moves.push((col, current_row));
                            break
                        } else {
                            break
                        }
                    }
                }
                // add all possible squares in current column in the negative direction until end of board or intersection with another game piece
                for row in (0..current_row).rev() {
                    if board.get_square_by_index(current_col, row).is_none() {
                        possible_moves.push((current_col, row));
                    } else {
                        if board.get_square_by_index(current_col, row).unwrap().side != self.side {
                            possible_moves.push((current_col, row));
                            break
                        } else {
                            break
                        }
                    }
                }
                // add all possible squares in current column in the positive direction until end of board or intersection with another game piece
                for row in current_row+1..8 {
                    if board.get_square_by_index(current_col, row).is_none() {
                        possible_moves.push((current_col, row));
                    } else {
                        if board.get_square_by_index(current_col, row).unwrap().side != self.side {
                            possible_moves.push((current_col, row));
                            break
                        } else {
                            break
                        }
                    }
                }
                // TODO handle castling
            },
            PieceType::Knight => {
                // double left moves
                if current_col > 1 {
                    // 2 left, 1 up
                    if current_row < 7 && (board.get_square_by_index(current_col - 2, current_row + 1).is_none() || board.get_square_by_index(current_col - 2, current_row + 1).unwrap().side != self.side) {
                            possible_moves.push((current_col - 2, current_row + 1));
                    }
                    // 2 left, 1 down
                    if current_row > 0 && (board.get_square_by_index(current_col - 2, current_row - 1).is_none() || board.get_square_by_index(current_col - 2, current_row - 1).unwrap().side != self.side) {
                        possible_moves.push((current_col - 2, current_row - 1));
                    }
                }
                // double right moves
                if current_col < 6 {
                    // 2 right, 1 up
                    if current_row < 7 && (board.get_square_by_index(current_col + 2, current_row + 1).is_none() || board.get_square_by_index(current_col + 2, current_row + 1).unwrap().side != self.side) {
                            possible_moves.push((current_col + 2, current_row + 1));
                    }
                    // 2 left, 1 down
                    if current_row > 0 && (board.get_square_by_index(current_col + 2, current_row - 1).is_none() || board.get_square_by_index(current_col + 2, current_row - 1).unwrap().side != self.side) {
                        possible_moves.push((current_col + 2, current_row - 1));
                    }
                }
                // double up moves
                if current_row < 6 {
                    // 2 up, 1 left
                    if current_col > 0 && (board.get_square_by_index(current_col - 1, current_row + 2).is_none() || board.get_square_by_index(current_col - 1, current_row + 2).unwrap().side != self.side) {
                            possible_moves.push((current_col - 1, current_row + 2));
                    }
                    // 2 up, 1 right
                    if current_col < 7 && (board.get_square_by_index(current_col + 1, current_row + 2).is_none() || board.get_square_by_index(current_col + 1, current_row + 2).unwrap().side != self.side) {
                        possible_moves.push((current_col + 1, current_row + 2));
                    }
                }
                // double down moves
                if current_row > 1 {
                    // 2 down, 1 left
                    if current_col > 0 && (board.get_square_by_index(current_col - 1, current_row - 2).is_none() || board.get_square_by_index(current_col - 1, current_row - 2).unwrap().side != self.side) {
                            possible_moves.push((current_col - 1, current_row - 2));
                    }
                    // 2 down, 1 left
                    if current_col < 7 && (board.get_square_by_index(current_col + 1, current_row - 2).is_none() || board.get_square_by_index(current_col + 1, current_row - 2).unwrap().side != self.side) {
                        possible_moves.push((current_col + 1, current_row - 2));
                    }
                }
            },
            PieceType::Bishop => {
                // diagonal moves from current column/row up and to left
                for diag_up_left in 1..current_col+1 {
                    let new_col = current_col - diag_up_left;
                    let new_row = current_row + diag_up_left;
                    if board.get_square_by_index(new_col, new_row).is_none() {
                        possible_moves.push((new_col, new_row));
                    } else {
                        if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                            break
                        } else {
                            break
                        }
                    }
                    if new_col == 0 || new_row == 7 {
                        break;
                    }
                }
                // diagonal moves from current column/row up and to right
                for diag_up_right in 1..8-current_col {
                    let new_col = current_col + diag_up_right;
                    let new_row = current_row + diag_up_right;
                    if board.get_square_by_index(new_col, new_row).is_none() {
                        possible_moves.push((new_col, new_row));
                    } else {
                        if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                            break
                        } else {
                            break
                        }
                    }
                    if new_col == 7 || new_row == 7 {
                        break;
                    }
                }
                // diagonal moves from current column/row down and to left
                for diag_down_left in 1..current_col+1 {
                    let new_col = current_col - diag_down_left;
                    let new_row = current_row - diag_down_left;
                    if board.get_square_by_index(new_col, new_row).is_none() {
                        possible_moves.push((new_col, new_row));
                    } else {
                        if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                            break
                        } else {
                            break
                        }
                    }
                    if new_col == 0 || new_row == 0 {
                        break;
                    }
                }
                // diagonal moves from current column/row down and to right
                for diag_down_right in 1..8-current_col {
                    let new_col = current_col + diag_down_right;
                    let new_row = current_row - diag_down_right;
                    if board.get_square_by_index(new_col, new_row).is_none() {
                        possible_moves.push((new_col, new_row));
                    } else {
                        if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                            break
                        } else {
                            break
                        }
                    }
                    if new_col == 7 || new_row == 0 {
                        break;
                    }
                }
            },
            PieceType::Queen => {
                // horizontal or vertical, no diagonal
                // add all possible squares in current row in the negative direction until end of board or intersection with another game piece
                for col in (0..current_col).rev() {
                    if board.get_square_by_index(col, current_row).is_none() {
                        possible_moves.push((col, current_row));
                    } else {
                        if board.get_square_by_index(col, current_row).unwrap().side != self.side {
                            possible_moves.push((col, current_row));
                            break
                        } else {
                            break
                        }
                    }
                }
                // add all possible squares in current row in the positive direction until end of board or intersection with another game piece
                for col in current_col+1..8 {
                    if board.get_square_by_index(col, current_row).is_none() {
                        possible_moves.push((col, current_row));
                    } else {
                        if board.get_square_by_index(col, current_row).unwrap().side != self.side {
                            possible_moves.push((col, current_row));
                            break
                        } else {
                            break
                        }
                    }
                }
                // add all possible squares in current column in the negative direction until end of board or intersection with another game piece
                for row in (0..current_row).rev() {
                    if board.get_square_by_index(current_col, row).is_none() {
                        possible_moves.push((current_col, row));
                    } else {
                        if board.get_square_by_index(current_col, row).unwrap().side != self.side {
                            possible_moves.push((current_col, row));
                            break
                        } else {
                            break
                        }
                    }
                }
                // add all possible squares in current column in the positive direction until end of board or intersection with another game piece
                for row in current_row+1..8 {
                    if board.get_square_by_index(current_col, row).is_none() {
                        possible_moves.push((current_col, row));
                    } else {
                        if board.get_square_by_index(current_col, row).unwrap().side != self.side {
                            possible_moves.push((current_col, row));
                            break
                        } else {
                            break
                        }
                    }
                }
                // diagonal moves
                // diagonal moves from current column/row up and to left
                for diag_up_left in 1..current_col+1 {
                    let new_col = current_col - diag_up_left;
                    let new_row = current_row + diag_up_left;
                    if board.get_square_by_index(new_col, new_row).is_none() {
                        possible_moves.push((new_col, new_row));
                    } else {
                        if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                            break
                        } else {
                            break
                        }
                    }
                    if new_col == 0 || new_row == 7 {
                        break;
                    }
                }
                // diagonal moves from current column/row up and to right
                for diag_up_right in 1..8-current_col {
                    let new_col = current_col + diag_up_right;
                    let new_row = current_row + diag_up_right;
                    if board.get_square_by_index(new_col, new_row).is_none() {
                        possible_moves.push((new_col, new_row));
                    } else {
                        if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                            break
                        } else {
                            break
                        }
                    }
                    if new_col == 7 || new_row == 7 {
                        break;
                    }
                }
                // diagonal moves from current column/row down and to left
                for diag_down_left in 1..current_col+1 {
                    let new_col = current_col - diag_down_left;
                    let new_row = current_row - diag_down_left;
                    if board.get_square_by_index(new_col, new_row).is_none() {
                        possible_moves.push((new_col, new_row));
                    } else {
                        if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                            break
                        } else {
                            break
                        }
                    }
                    if new_col == 0 || new_row == 0 {
                        break;
                    }
                }
                // diagonal moves from current column/row down and to right
                for diag_down_right in 1..8-current_col {
                    let new_col = current_col + diag_down_right;
                    let new_row = current_row - diag_down_right;
                    if board.get_square_by_index(new_col, new_row).is_none() {
                        possible_moves.push((new_col, new_row));
                    } else {
                        if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                            break
                        } else {
                            break
                        }
                    }
                    if new_col == 7 || new_row == 0 {
                        break;
                    }
                }
            },
            PieceType::King => {
                for col_shift in 0..3 {
                    for row_shift in 0..3 {
                        // ignore any moves which would move off the board
                        if (col_shift == 0 && current_col == 0) || (row_shift == 0 && current_row == 0) || (col_shift == 2 && current_col == 7) || (row_shift == 2 && current_row == 7) {
                            continue;
                        }

                        // ignore the center shift, as it's the current square
                        if col_shift == 2 && row_shift == 2 {
                            continue;
                        }

                        let new_col = current_col + col_shift - 1;
                        let new_row = current_row + row_shift - 1;

                        if board.get_square_by_index(new_col, new_row).is_none() {
                            possible_moves.push((new_col, new_row));
                        } else if board.get_square_by_index(new_col, new_row).unwrap().side != self.side {
                            possible_moves.push((new_col, new_row));
                        }
                    }
                }
            },
        }

        // TODO validate move further by making sure the move doesn't result in a checkmate
        return possible_moves
    }

    pub fn validate_move(self: &Self, board: &ChessBoard, desired_position: (usize, usize)) -> bool {
        let board_square_contents = board.get_square_by_index(desired_position.0, desired_position.1);
        let current_col = self.position.0;
        let current_row = self.position.1;
        let valid_move = match self.piece_type {
            PieceType::Pawn => {
                todo!()
            },
            PieceType::Rook => {
                (desired_position.0 == current_col && (desired_position.1 as i64 - current_row as i64).abs() > 0)
                ||
                (desired_position.1 == current_row && (desired_position.0 as i64 - current_col as i64).abs() > 0)
            },
            PieceType::Knight => todo!(),
            PieceType::Bishop => {
                todo!()
            },
            PieceType::Queen => todo!(),
            PieceType::King => todo!(),
        };

        return valid_move
    }
}