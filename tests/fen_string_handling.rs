use chessbot_lib::gamelogic::board::ChessBoard;

#[test]
fn starting_position_fen_parsed_correctly() {
    let fen_board = ChessBoard::new_from_forsyth_edwards("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0".to_string());
    assert!(fen_board.is_ok());
    let default_board = ChessBoard::new();
    assert_eq!(default_board.get_board_state_hash(), fen_board.unwrap().get_board_state_hash())
}

#[test]
fn default_board_fen_correct_conversion() {
    let default_board = ChessBoard::new();
    assert_eq!(default_board.to_forsyth_edwards(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0".to_string());
}