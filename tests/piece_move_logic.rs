#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, RwLock};

use chessbot_lib::gamelogic::{board::ChessBoard, pieces::{PieceType, ChessPiece}, index_pair_to_name, MoveType, ChessMove, name_to_index_pair, Side};


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
    static ref CUSTOM_BOARD: Arc<RwLock<ChessBoard>> = {
        let mut custom_setup_squares: [[Option<ChessPiece>; 8]; 8] = Default::default();
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
        custom_setup_squares[4][0] = Some(ChessPiece { position: (4,0), side: Side::White, piece_type: PieceType::Rook});
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
        custom_setup_squares[7][0] = Some(ChessPiece { position: (7,0), side: Side::White, piece_type: PieceType::King});
        custom_setup_squares[7][2] = Some(ChessPiece { position: (7,2), side: Side::White, piece_type: PieceType::Knight});  // changed from pawn to Knight
        custom_setup_squares[7][6] = Some(ChessPiece { position: (7,6), side: Side::Black, piece_type: PieceType::Pawn});
        custom_setup_squares[7][7] = Some(ChessPiece { position: (7,7), side: Side::Black, piece_type: PieceType::Rook});

        let board = ChessBoard::new_with_squares(custom_setup_squares);

        // wrap board in an Arc to send between threads since tests are parallel, and use a RwLock to allow simulataneous reading
        Arc::new(RwLock::new(board))
    };
}


#[test]
fn black_g7_pawn_can_double_advance() {
    let board = CUSTOM_BOARD.read().unwrap();
    let pawn = board.get_square_by_name("g7".to_string()).unwrap();
    assert!(pawn.is_some());
    let moves = pawn.unwrap().get_moves(&board).iter().map(|m| index_pair_to_name(m.destination.0, m.destination.1).unwrap()).collect::<Vec<String>>();
    assert_eq!(moves, ["g5", "g6"])
}


#[test]
fn white_h3_knight_is_pinned() {
    let board = CUSTOM_BOARD.read().unwrap();
    let piece = board.get_square_by_name("h3".to_string()).unwrap();
    assert!(piece.is_some());
    let moves = piece.unwrap().get_moves(&board);
    assert_eq!(moves, [])
}


#[test]
fn en_passant_capture() {
    let original_board = CUSTOM_BOARD.read().unwrap();
    let mut board = original_board.clone();

    let black_pawn_opt = board.get_square_by_name("g7".to_string()).unwrap();
    assert!(black_pawn_opt.is_some());

    let black_move = ChessMove {
        from_square: name_to_index_pair("g7".to_string()).unwrap(),
        destination: (6,4),
        move_type: MoveType::Standard,
        captures: None
    };

    // move black pawn double forward opening up to en passant move
    assert!(board.perform_move(&black_move).is_ok());

    // assert pawn actually moved
    assert!(board.get_square_by_name("g7".to_string()).unwrap().is_none());

    // assert white pawn exists
    assert!(board.get_square_by_name("f5".to_string()).unwrap().is_some());

    // assert white pawn can perform en passant move
    let white_move = ChessMove {
        from_square: name_to_index_pair("f5".to_string()).unwrap(),
        destination: (6,5),
        move_type: MoveType::EnPassant,
        captures: Some((6, 4))
    };
    assert!(board.perform_move(&white_move).is_ok());

    // white pawn has moved
    assert!(board.get_square_by_name("f5".to_string()).unwrap().is_none());
    // assert black pawn is now captured
    assert!(board.get_square_by_name("g5".to_string()).unwrap().is_none());
    // white pawn has moved diagonally to correct square after capture
    assert!(board.get_square_by_name("g6".to_string()).unwrap().is_some());
}


#[test]
fn white_pawn_knight_capture() {
    let original_board = CUSTOM_BOARD.read().unwrap();
    let mut board = original_board.clone();

    let white_pawn_opt = board.get_square_by_name("g2".to_string()).unwrap();
    assert!(white_pawn_opt.is_some());

    let white_pawn = white_pawn_opt.unwrap();
    let white_move_res = white_pawn.get_specific_move(&board, name_to_index_pair("f3".to_string()).unwrap());
    assert!(white_move_res.is_ok());

    let moved = board.perform_move(&white_move_res.unwrap());
    assert!(moved.is_ok());

    assert!(board.get_square_by_name("g2".to_string()).unwrap().is_none());

    let target_square_opt = board.get_square_by_name("f3".to_string()).unwrap();
    assert!(target_square_opt.is_some());

    let target_square_piece = target_square_opt.unwrap();
    assert_eq!(target_square_piece.piece_type, PieceType::Pawn);
    assert_eq!(target_square_piece.side, Side::White);
}