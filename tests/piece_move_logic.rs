#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex, Arc, RwLock};

use chessbot_lib::gamelogic::{board::ChessBoard, pieces::{Side, PieceType, ChessPiece}, index_pair_to_name};

/*
Setup a custom board state (all flags set to newgame) to evaluate move logic.

8 ·K···B·R
7 pp···ppp
6 ··p····Q
5 ···ppp··
4 Qp··B···
3 p····N·N
2 ···p·pp·
1 R·B·K··R
  abcdefgh
*/
lazy_static! {
    static ref custom_board: Arc<RwLock<ChessBoard>> = {
        let mut custom_setup_squares = [[None; 8]; 8];
        custom_setup_squares[0][0] = Some(ChessPiece { position: (0,0), side: Side::White, piece_type: PieceType::Rook});
        custom_setup_squares[0][2] = Some(ChessPiece { position: (0,2), side: Side::White, piece_type: PieceType::Pawn});
        custom_setup_squares[0][3] = Some(ChessPiece { position: (0,3), side: Side::White, piece_type: PieceType::Queen});
        custom_setup_squares[0][6] = Some(ChessPiece { position: (0,6), side: Side::Black, piece_type: PieceType::Pawn});
        custom_setup_squares[1][3] = Some(ChessPiece { position: (1,3), side: Side::White, piece_type: PieceType::Pawn});
        custom_setup_squares[1][6] = Some(ChessPiece { position: (1,6), side: Side::Black, piece_type: PieceType::Pawn});
        custom_setup_squares[1][7] = Some(ChessPiece { position: (1,7), side: Side::Black, piece_type: PieceType::King});
        custom_setup_squares[2][0] = Some(ChessPiece { position: (2,0), side: Side::White, piece_type: PieceType::Bishop});
        custom_setup_squares[2][5] = Some(ChessPiece { position: (2,5), side: Side::Black, piece_type: PieceType::Pawn}); // moved 1 down
        custom_setup_squares[3][1] = Some(ChessPiece { position: (3,1), side: Side::White, piece_type: PieceType::Pawn});
        custom_setup_squares[3][4] = Some(ChessPiece { position: (3,4), side: Side::White, piece_type: PieceType::Pawn});
        custom_setup_squares[4][0] = Some(ChessPiece { position: (4,0), side: Side::White, piece_type: PieceType::King});
        custom_setup_squares[4][3] = Some(ChessPiece { position: (4,3), side: Side::Black, piece_type: PieceType::Bishop}); // added piece
        custom_setup_squares[4][4] = Some(ChessPiece { position: (4,4), side: Side::Black, piece_type: PieceType::Pawn});
        custom_setup_squares[5][1] = Some(ChessPiece { position: (5,1), side: Side::White, piece_type: PieceType::Pawn});
        custom_setup_squares[5][2] = Some(ChessPiece { position: (5,2), side: Side::Black, piece_type: PieceType::Knight});
        custom_setup_squares[5][4] = Some(ChessPiece { position: (5,4), side: Side::White, piece_type: PieceType::Pawn});
        custom_setup_squares[5][6] = Some(ChessPiece { position: (5,6), side: Side::Black, piece_type: PieceType::Pawn});
        custom_setup_squares[5][7] = Some(ChessPiece { position: (5,7), side: Side::Black, piece_type: PieceType::Bishop});
        custom_setup_squares[6][1] = Some(ChessPiece { position: (6,1), side: Side::White, piece_type: PieceType::Pawn});
        custom_setup_squares[7][5] = Some(ChessPiece { position: (7,5), side: Side::Black, piece_type: PieceType::Queen});  // moved over
        custom_setup_squares[6][6] = Some(ChessPiece { position: (6,6), side: Side::Black, piece_type: PieceType::Pawn});
        custom_setup_squares[7][0] = Some(ChessPiece { position: (7,0), side: Side::White, piece_type: PieceType::Rook});
        custom_setup_squares[7][2] = Some(ChessPiece { position: (7,2), side: Side::White, piece_type: PieceType::Knight});  // changed from pawn to Knight
        custom_setup_squares[7][6] = Some(ChessPiece { position: (7,6), side: Side::Black, piece_type: PieceType::Pawn});
        custom_setup_squares[7][7] = Some(ChessPiece { position: (7,7), side: Side::Black, piece_type: PieceType::Rook});

        let board = ChessBoard {
            squares: custom_setup_squares,
            state: Default::default()
        };

        Arc::new(RwLock::new(board))
    };
}

#[test]
fn black_g7_pawn_can_double_advance() {
    let board = custom_board.read().unwrap();
    let pawn = board.get_square_by_name("g7".to_string()).unwrap();
    assert!(pawn.is_some());
    let moves = pawn.unwrap().get_moves(&board).iter().map(|pair| index_pair_to_name(pair.0, pair.1).unwrap()).collect::<Vec<String>>();
    assert_eq!(moves, ["g5", "g6"])
}

#[test]
fn white_h3_knight_exposed_check_no_valid_moves() {
    let board = custom_board.read().unwrap();
    let piece = board.get_square_by_name("h3".to_string()).unwrap();
    assert!(piece.is_some());
    let moves = piece.unwrap().get_moves(&board);
    assert_eq!(moves, [])
}

// TODO more tests
// move for all pieces
// captures
// passant
// castling