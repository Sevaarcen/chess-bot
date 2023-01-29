use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Display;
use std::hash::Hash;
use std::hash::Hasher;

use super::ChessError;
use super::ChessMove;
use super::GameEnd;
use super::MoveType;
use super::Side;
use super::index_pair_to_name;
use super::name_to_index_pair;
use super::pieces::{ChessPiece, PieceType};

use colored::*;

#[derive(Clone, Debug)]
pub struct ChessBoard {
    pub squares: [[Option<ChessPiece>; 8]; 8], // 0,0 = a1, 7,7 = h8
    pub state: BoardStateFlags,
    board_state_counts: HashMap<u64, usize>,
    pub move_list: Vec<ChessMove>
}

#[derive(Copy, Clone, Debug)]
pub struct BoardStateFlags {
    pub white_castle_queenside: bool,
    pub white_castle_kingside: bool,
    pub black_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub en_passant_column: Option<usize>,
    pub current_turn: Side
}

impl Default for BoardStateFlags {
    fn default() -> Self {
        Self {
            white_castle_queenside: true,
            white_castle_kingside: true,
            black_castle_queenside: true,
            black_castle_kingside: true,
            en_passant_column: Default::default(),
            current_turn: Default::default()
        }
    }
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
            state: BoardStateFlags { ..Default::default() },  // start with all flags false
            board_state_counts: HashMap::new(),
            move_list: Vec::new()
        }
    }

    pub fn new_with_squares(setup: [[Option<ChessPiece>; 8]; 8]) -> Self {
        ChessBoard {
            squares: setup,  // 2d array of columns and rows
            state: BoardStateFlags { ..Default::default() },  // start with all flags false
            board_state_counts: HashMap::new(),
            move_list: Vec::new()
        }
    }

    pub fn new_from_forsyth_edwards(fen_string: String) -> Result<Self, ChessError> {
        todo!()
    }

    pub fn to_forsyth_edwards(self: &Self) -> String {
        // figure out where all the pieces are
        let mut piece_placement = String::new();
        for row in 0..=7 {
            let mut empty_squares = 0;
            for col in 0..=7 {
                let s = self.squares[col][7 - row];  // we start on rank 8 and go to rank 1
                match s {
                    Some(p) => {
                        if empty_squares > 0 {
                            piece_placement.push(char::from_u32(48 + empty_squares).unwrap());
                            empty_squares = 0;
                        }
                        piece_placement.push(p.into());
                    },
                    None => empty_squares += 1
                }
            }
            if empty_squares > 0 {
                piece_placement.push(char::from_u32(48 + empty_squares).unwrap());
            }
            if row != 7 {
                piece_placement.push('/');
            }
        }

        // get current turn
        let active_side = match self.state.current_turn {
            Side::White => 'w',
            Side::Black => 'b',
        };

        // determine what, if any, castling ability players have (ignoring temp restrictions)
        let mut castling_ability = String::new();
        if self.state.white_castle_kingside {
            castling_ability.push('K');
        }
        if self.state.white_castle_queenside {
            castling_ability.push('Q');
        }
        if self.state.black_castle_kingside {
            castling_ability.push('k');
        }
        if self.state.black_castle_queenside {
            castling_ability.push('q');
        }
        if castling_ability.is_empty() {
            castling_ability = "-".to_string();
        }

        // figure out en passant target square if applicable
        let en_passant_sqr = match self.state.en_passant_column {
            // the target row is always going to be the same depending on the opponent side
            Some(c) => match !self.state.current_turn {
                Side::White => index_pair_to_name(c, 2).unwrap(),
                Side::Black => index_pair_to_name(c, 5).unwrap(),
            },
            None => "-".to_string(),
        };

        let halfmove_clock = 0;  // TODO do I even need these?
        let fullmove_click = 0;

        format!("{} {} {} {} {} {}", piece_placement, active_side, castling_ability, en_passant_sqr, halfmove_clock, fullmove_click)
    }

    pub fn get_total_materials(self: &Self, side: Side) -> usize {
        self.squares.iter()
            .map(
                |row|
                row.iter()
                    .filter(|square| square.is_some())
                    .filter(|square| square.unwrap().side == side)
                    .map(|square| square.unwrap().get_material())
                    .sum::<usize>()
            )
            .sum::<usize>()
    }

    pub fn get_square_by_index(self: &Self, column: usize, row: usize) -> Option<ChessPiece> {
        // TODO change to result in case given indexes are known to be out of range? or just deal w/ potential run-time error
        self.squares[column][row]
    }

    pub fn get_square_by_position(self: &Self, position: (usize, usize)) -> Option<ChessPiece> {
        self.get_square_by_index(position.0, position.1)
    }

    pub fn get_square_by_name(self: &Self, square_name: String) -> Result<Option<ChessPiece>, ChessError> {
        let (column_index, row_index) = name_to_index_pair(square_name)?;
        Ok(self.get_square_by_index(column_index, row_index))
    }

    pub fn perform_move(self: &mut Self, chess_move: &ChessMove) -> Result<(), ()> {
        let current_position = chess_move.from_square;
        let mut piece = self.get_square_by_index(current_position.0, current_position.1).expect(format!("Tried to get a piece at position {:?} but piece didn't exist", current_position).as_str());
        let dest_col = chess_move.destination.0;
        let dest_row = chess_move.destination.1;


        // handle special moves
        match chess_move.move_type {
            MoveType::EnPassant => {
                let captured = match piece.side {
                    Side::White => self.get_square_by_index(dest_col, dest_row - 1).expect(format!("Tried to perform en passant capture at position but piece didn't exist: {:#?}", chess_move).as_str()),
                    Side::Black => self.get_square_by_index(dest_col, dest_row + 1).expect(format!("Tried to perform en passant capture at position but piece didn't exist: {:#?}", chess_move).as_str()),
                };
                self.squares[captured.position.0][captured.position.1] = None;
                self.state.en_passant_column = None;
            },
            MoveType::DoubleAdvance => {
                self.state.en_passant_column = Some(dest_col);
            },
            MoveType::Promotion => {
                piece.piece_type = PieceType::Queen; // there's no reason why we would want a different piece type
                self.state.en_passant_column = None;
            },
            MoveType::Castle => {
                // the normal move of the king will be performed, but then we want to create a move for the rook and move it too
                let (castle_from_col, castle_dest_col) = match dest_col == 1 {
                    true => (0, 2),
                    false => (7, 5)
                };
                let castle_move = ChessMove {
                    from_square: (castle_from_col, dest_row),
                    destination: (castle_dest_col, dest_row),
                    move_type: MoveType::Standard,
                    captures: None
                };
                self.perform_move(&castle_move)?;
            },
            _ => {
                self.state.en_passant_column = None;
            }
        }
        // handle board state flags when the rook moves off their starting square, removing the possibility for castling with that rook
        if piece.piece_type == PieceType::Rook {
            match current_position {
                // white queen's rook
                (0, 0) => self.state.white_castle_queenside = false,
                // white king's rook
                (7, 0) => self.state.white_castle_kingside = false,
                // black queen's rook
                (0, 7) => self.state.black_castle_queenside = false,
                // black king's rook
                (7, 7) => self.state.black_castle_kingside = false,
                // if it's any move other than off the starting square, no flags need to be changed
                _ => ()
            }
        }
        // if the king is what moved, unset the flags to disable castling
        if piece.piece_type == PieceType::King {
            match piece.side {
                Side::White => {
                    self.state.white_castle_kingside = false;
                    self.state.white_castle_queenside = false;
                },
                Side::Black => {
                    self.state.black_castle_kingside = false;
                    self.state.black_castle_queenside = false;
                },
            }
        }

        // move piece from current position to destination
        piece.position = chess_move.destination;
        self.squares[current_position.0][current_position.1] = None;
        self.squares[dest_col][dest_row] = Some(piece);

        Ok(())
    }

    pub fn record_board_state(self: &mut Self) -> () {
        let new_state_hash = self.get_board_state_hash();
        let state_seen_count = self.board_state_counts.entry(new_state_hash).or_default();
        *state_seen_count = *state_seen_count + 1;
    }

    pub fn perform_move_and_record(self: &mut Self, chess_move: &ChessMove) -> Result<(), ()> {
        self.state.current_turn = !self.get_square_by_position(chess_move.from_square).unwrap().side;
        self.perform_move(chess_move)?;
        self.record_board_state();
        self.move_list.push(chess_move.clone());
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

    pub fn get_square_threats(self: &Self, side: Side, square: (usize, usize)) -> Vec<ChessPiece> {
        let mut threateners = Vec::new();

        for col in self.squares {
            for cell in col {
                if cell.is_none() {
                    continue;
                }
                let piece = cell.unwrap();
                if piece.side != side {
                    continue;
                }
                let piece_threats = piece.get_threats(&self);
                if piece_threats.contains(&square) {
                    threateners.push(piece)
                }
            }
        }
        threateners
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

    pub fn get_board_state_hash(self: &Self) -> u64 {
        let board_formatted = format!("{}", self);
        let mut hasher = DefaultHasher::new();
        board_formatted.hash(&mut hasher);
        hasher.finish()
    }

    /// Checks if there's a game ending state for the given board.
    ///
    /// Reference: https://www.chess.com/article/view/how-chess-games-can-end-8-ways-explained
    pub fn is_game_over(self: &Self, current_turn: Side) -> Option<GameEnd> {
        match current_turn {
            Side::White => {
                let white_is_checked = self.is_checked(Side::White);
                let white_has_no_moves = self.get_all_moves(Side::White).is_empty();
                if white_is_checked && white_has_no_moves {
                    return Some(GameEnd::BlackVictory("Checkmate".to_string()));
                }
                if white_has_no_moves {
                    // If there are no valid moves which White can make, that means the game is in a draw
                    return Some(GameEnd::Draw("White has no valid moves".to_string()));
                }
            },
            Side::Black => {
                let black_is_checked = self.is_checked(Side::Black);
                let black_has_no_moves = self.get_all_moves(Side::Black).is_empty();
                if black_is_checked && black_has_no_moves {
                    // White achieved Checkmate if Black remains in Check and has no valid moves remaining to escape
                    return Some(GameEnd::WhiteVictory("Checkmate".to_string()));
                } else if black_has_no_moves {
                    // If there are no valid moves which White can make, that means the game is in a draw
                    return Some(GameEnd::Draw("Black has no valid moves".to_string()));
                }
            }
        }

        // otherwise check for stalemate / insufficient materials

        // Check for insufficient material game ending. This occurs when one side only has a king, or both sides have their king plus a minot piece (bishop or knight)
        let white_pieces = self.get_all_pieces(Side::White);
        let black_pieces = self.get_all_pieces(Side::Black);

        // Game is a draw if both sides are left with only the king
        if white_pieces.len() == 1 && black_pieces.len() == 1 {
            return Some(GameEnd::Draw("Stalemate".to_string()));
        }
        // Game ends in a draw if White only has their king, ...
        else if white_pieces.len() == 1 {
            // and a Knight/Bishop
            if black_pieces.len() == 2 && black_pieces.iter().find(|p| p.piece_type != PieceType::King).unwrap().get_material() == 3 {
                return Some(GameEnd::Draw("Insufficient material".to_string()));
            }
            // or just two Knights
            else if black_pieces.len() == 3 && black_pieces.iter().filter(|p| p.piece_type != PieceType::King).filter(|p| p.piece_type == PieceType::Knight).nth(1).is_some() {
                return Some(GameEnd::Draw("Insufficient material".to_string()));
            }
        }
        // Game ends in a draw if Black only has their King, ...
        else if black_pieces.len() == 1 {
            // and a Knight/Bishop
            if white_pieces.len() == 2 && white_pieces.iter().find(|p| p.piece_type != PieceType::King).unwrap().get_material() == 3 {
                return Some(GameEnd::Draw("Insufficient material".to_string()));
            }
            // or has just 2 Knights
            else if white_pieces.len() == 3 && white_pieces.iter().filter(|p| p.piece_type != PieceType::King).filter(|p| p.piece_type == PieceType::Knight).nth(1).is_some() {
                return Some(GameEnd::Draw("Insufficient material".to_string()));
            }
        }
        // Game ends in a Draw if both sides have their Kings and a Knight/Bishop piece each
        else if white_pieces.len() == 2 && black_pieces.len() == 2 && white_pieces.iter().find(|p| p.piece_type != PieceType::King).unwrap().get_material() == 3 && black_pieces.iter().find(|p| p.piece_type != PieceType::King).unwrap().get_material() == 3 {
            return Some(GameEnd::Draw("Insufficient material".to_string()));
        }

        // check for draw by repition. If any board state hash has occured 3 or more times, it's a draw.
        if self.board_state_counts.values().find(|v| **v == 3).is_some() {
            return Some(GameEnd::Draw("Draw by repetition".to_string()));
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
                            // to swap print style to non-unicode, comment out above and replace with below
                            // Side::White => match piece.piece_type {
                            //     PieceType::Pawn => "wP",
                            //     PieceType::Rook => "wR",
                            //     PieceType::Knight => "wN",
                            //     PieceType::Bishop => "wB",
                            //     PieceType::Queen => "wQ",
                            //     PieceType::King => "wK",
                            // }.white(),
                            // Side::Black => match piece.piece_type {
                            //     PieceType::Pawn => "bP",
                            //     PieceType::Rook => "bR",
                            //     PieceType::Knight => "bN",
                            //     PieceType::Bishop => "bB",
                            //     PieceType::Queen => "bQ",
                            //     PieceType::King => "bK",
                            // }.blue(),
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
                // write!(f, "{}", char)?; // write w/ no background
            }
            write!(f, "\n")?;
        };
        write!(f, "  {}\n", "a b c d e f g h".black())?;
        Ok(())
    }
}