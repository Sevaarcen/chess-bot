use chessbot_lib::gamelogic::{board::ChessBoard, pieces::PieceType, MoveType};

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

#[test]
fn fen_string_black_queen_castle() {
    let board_res = ChessBoard::from_forsyth_edwards("r3kb1r/1p2pppp/1qn4n/p1pp1P2/5PP1/1P5P/P1PP3R/RNBQKB2 b Qkq - 0 0".to_string());
    assert!(board_res.is_ok());
    let mut board = board_res.unwrap();
    let king = board.get_square_by_name("e8".to_string()).unwrap();
    assert!(king.is_some());
    let castle_move_res = king.unwrap().get_specific_move(&board, (2, 7));
    assert!(castle_move_res.is_ok());
    let castle_move = castle_move_res.unwrap();
    assert!(matches!(castle_move.move_type, chessbot_lib::gamelogic::MoveType::Castle));
    let performed_move = board.perform_move_and_record(&castle_move);
    assert!(performed_move.is_ok());
    let moved_king = board.get_square_by_name("c8".to_string()).unwrap();
    assert!(moved_king.is_some());
    assert!(matches!(moved_king.unwrap().piece_type, PieceType::King));
    let moved_castle = board.get_square_by_name("d8".to_string()).unwrap();
    assert!(moved_castle.is_some());
    assert!(matches!(moved_castle.unwrap().piece_type, PieceType::Rook));
}

#[test]
fn fen_string_black_cannot_castle_h8_missing_rook() {
    let board_res = ChessBoard::from_forsyth_edwards("1r2k3/7R/1p1p4/2pPn2p/p1P2p2/P1P5/2KN1P2/8 b k - 0 0".to_string());
    assert!(board_res.is_ok());
    let board = board_res.unwrap();
    let king = board.get_square_by_name("e8".to_string()).unwrap();
    assert!(king.is_some());
    let king_moves = king.unwrap().get_moves(&board);
    assert!(king_moves.iter().find(|m| m.move_type == MoveType::Castle).is_none())
}