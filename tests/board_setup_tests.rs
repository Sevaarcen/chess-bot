use chessbot_lib::gamelogic::{board::ChessBoard, pieces::{Side, PieceType}};

#[test]
fn get_square_a1_ok_white_rook() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("a1".to_string());
    assert!(piece.is_ok());
    let piece_un = piece.unwrap();
    assert!(piece_un.is_some());
    assert_eq!(piece_un.unwrap().position, (0,0));
    assert!(matches!(piece_un.unwrap().piece_type, PieceType::Rook));
    assert!(matches!(piece_un.unwrap().side, Side::White));
}

#[test]
fn get_square_a2_ok_white_pawn() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("a2".to_string());
    assert!(piece.is_ok());
    let piece_un = piece.unwrap();
    assert!(piece_un.is_some());
    assert_eq!(piece_un.unwrap().position, (0,1));
    assert!(matches!(piece_un.unwrap().piece_type, PieceType::Pawn));
    assert!(matches!(piece_un.unwrap().side, Side::White));
}

#[test]
fn get_square_h2_ok_white_pawn() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("h2".to_string());
    assert!(piece.is_ok());
    let piece_un = piece.unwrap();
    assert!(piece_un.is_some());
    assert_eq!(piece_un.unwrap().position, (7,1));
    assert!(matches!(piece_un.unwrap().piece_type, PieceType::Pawn));
    assert!(matches!(piece_un.unwrap().side, Side::White));
}

#[test]
fn get_square_e3_ok_none() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("e3".to_string());
    assert!(piece.is_ok());
    let piece_un = piece.unwrap();
    assert!(piece_un.is_none());
}

#[test]
fn get_square_h8_ok_black_root() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("h8".to_string());
    assert!(piece.is_ok());
    let piece_un = piece.unwrap();
    assert!(piece_un.is_some());
    assert_eq!(piece_un.unwrap().position, (7,7));
    assert!(matches!(piece_un.unwrap().piece_type, PieceType::Rook));
    assert!(matches!(piece_un.unwrap().side, Side::Black));
}

#[test]
fn get_square_a7_ok_black_pawn() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("a7".to_string());
    assert!(piece.is_ok());
    let piece_un = piece.unwrap();
    assert!(piece_un.is_some());
    assert_eq!(piece_un.unwrap().position, (0,6));
    assert!(matches!(piece_un.unwrap().piece_type, PieceType::Pawn));
    assert!(matches!(piece_un.unwrap().side, Side::Black));
}

#[test]
fn get_square_h7_ok_black_pawn() {
    let board = ChessBoard::new();
    let piece = board.get_square_by_name("h7".to_string());
    assert!(piece.is_ok());
    let piece_un = piece.unwrap();
    assert!(piece_un.is_some());
    assert_eq!(piece_un.unwrap().position, (7,6));
    assert!(matches!(piece_un.unwrap().piece_type, PieceType::Pawn));
    assert!(matches!(piece_un.unwrap().side, Side::Black));
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