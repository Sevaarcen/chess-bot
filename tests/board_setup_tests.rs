use chessbot_lib::gamelogic::{board::ChessBoard, pieces::PieceType, Side};

#[test]
fn get_square_a1_ok_white_rook() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("a1".to_string());
    assert!(piece.is_ok());
    let piece_opt = piece.unwrap();
    assert!(piece_opt.is_some());
    let piece = piece_opt.unwrap();
    assert_eq!(piece.position, (0,0));
    assert!(matches!(piece.piece_type, PieceType::Rook));
    assert!(matches!(piece.side, Side::White));
}

#[test]
fn get_square_a2_ok_white_pawn() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("a2".to_string());
    assert!(piece.is_ok());
    let piece_opt = piece.unwrap();
    assert!(piece_opt.is_some());
    let piece = piece_opt.unwrap();
    assert_eq!(piece.position, (0,1));
    assert!(matches!(piece.piece_type, PieceType::Pawn));
    assert!(matches!(piece.side, Side::White));
}

#[test]
fn get_square_h2_ok_white_pawn() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("h2".to_string());
    assert!(piece.is_ok());
    let piece_opt = piece.unwrap();
    assert!(piece_opt.is_some());
    let piece = piece_opt.unwrap();
    assert_eq!(piece.position, (7,1));
    assert!(matches!(piece.piece_type, PieceType::Pawn));
    assert!(matches!(piece.side, Side::White));
}

#[test]
fn get_square_e3_ok_none() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("e3".to_string());
    assert!(piece.is_ok());
    let piece = piece.unwrap();
    assert!(piece.is_none());
}

#[test]
fn get_square_h8_ok_black_root() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("h8".to_string());
    assert!(piece.is_ok());
    let piece_opt = piece.unwrap();
    assert!(piece_opt.is_some());
    let piece = piece_opt.unwrap();
    assert_eq!(piece.position, (7,7));
    assert!(matches!(piece.piece_type, PieceType::Rook));
    assert!(matches!(piece.side, Side::Black));
}

#[test]
fn get_square_a7_ok_black_pawn() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("a7".to_string());
    assert!(piece.is_ok());
    let piece_opt = piece.unwrap();
    assert!(piece_opt.is_some());
    let piece = piece_opt.unwrap();
    assert_eq!(piece.position, (0,6));
    assert!(matches!(piece.piece_type, PieceType::Pawn));
    assert!(matches!(piece.side, Side::Black));
}

#[test]
fn get_square_h7_ok_black_pawn() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("h7".to_string());
    assert!(piece.is_ok());
    let piece_opt = piece.unwrap();
    assert!(piece_opt.is_some());
    let piece = piece_opt.unwrap();
    assert_eq!(piece.position, (7,6));
    assert!(matches!(piece.piece_type, PieceType::Pawn));
    assert!(matches!(piece.side, Side::Black));
}

#[test]
fn get_square_z1_err() {
    let board = ChessBoard::new();
    assert!(board.get_square_by_name("z1".to_string()).is_err());
}

#[test]
fn get_square_a9_err() {
    let board = ChessBoard::new();
    assert!(board.get_square_by_name("a9".to_string()).is_err());
}

#[test]
fn get_square_abc123_err() {
    let board = ChessBoard::new();
    assert!(board.get_square_by_name("abc123".to_string()).is_err());
}

#[test]
fn fen_string_white_queen_castle() {
    let board_res = ChessBoard::from_forsyth_edwards("2r1kb1r/p1p1p1pp/2p1p3/3n2B1/2NP4/2P2P1b/PP3P1P/R3K2R w KQk - 0 0".to_string());
    assert!(board_res.is_ok());
    let mut board = board_res.unwrap();
    let king = board.get_square_by_name("e1".to_string()).unwrap();
    assert!(king.is_some());
    let castle_move_res = king.unwrap().get_specific_move(&board, (2, 0));
    assert!(castle_move_res.is_ok());
    let castle_move = castle_move_res.unwrap();
    assert!(matches!(castle_move.move_type, chessbot_lib::gamelogic::MoveType::Castle));
    let performed_move = board.perform_move_and_record(&castle_move);
    assert!(performed_move.is_ok());
    let moved_king = board.get_square_by_name("c1".to_string()).unwrap();
    assert!(moved_king.is_some());
    assert!(matches!(moved_king.unwrap().piece_type, PieceType::King));
    let moved_castle = board.get_square_by_name("d1".to_string()).unwrap();
    assert!(moved_castle.is_some());
    assert!(matches!(moved_castle.unwrap().piece_type, PieceType::Rook));
}