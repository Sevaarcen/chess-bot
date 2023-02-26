use chessbot_lib::gamelogic::board::ChessBoard;

#[test]
fn starting_position_fen_parsed_correctly() {
    let fen_board = ChessBoard::from_forsyth_edwards("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0".to_string());
    assert!(fen_board.is_ok());
    let default_board = ChessBoard::new();
    assert_eq!(default_board.get_board_state_hash(), fen_board.unwrap().get_board_state_hash())
}

#[test]
fn default_board_fen_correct_conversion() {
    let default_board = ChessBoard::new();
    assert_eq!(default_board.to_forsyth_edwards(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0".to_string());
}

#[test]
fn valid_fen_string_1() {
    let board = ChessBoard::from_forsyth_edwards("8/8/8/8/8/8/8/8 w - - 0 0".to_string());
    assert!(board.is_ok())
}

#[test]
fn invalid_fen_string_1() {
    let board = ChessBoard::from_forsyth_edwards("This ain't no FEN string!".to_string());
    assert!(board.is_err())
}

#[test]
fn invalid_fen_string_2() {
    let board = ChessBoard::from_forsyth_edwards("YEET 1 2 3 4 5".to_string());
    assert!(board.is_err())
}

#[test]
fn invalid_fen_string_3() {
    let board = ChessBoard::from_forsyth_edwards("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR - - - - -".to_string());
    assert!(board.is_err())
}

#[test]
fn invalid_fen_string_4() {
    let board = ChessBoard::from_forsyth_edwards("Z7/8/8/8/8/8/8/8 w - - 0 0".to_string());
    assert!(board.is_err())
}

#[test]
fn invalid_fen_string_5() {
    let board = ChessBoard::from_forsyth_edwards("8/8/8/8/8/8/8/8".to_string());
    assert!(board.is_err())
}

#[test]
fn fen_string_en_passant() {
    let board_res = ChessBoard::from_forsyth_edwards("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3".to_string());
    assert!(board_res.is_ok());
    let board = board_res.unwrap();
    assert_eq!(board.state.en_passant_column, Some(3))
}

#[test]
fn fen_string_parse_1() {
    let board = ChessBoard::from_forsyth_edwards("r1b1kbnr/1ppp1p1p/p1n3p1/4p3/2Q1P1Pq/7N/PPPP1P1P/RNB1KB1R w KQkq - 0 0".to_string());
    assert!(board.is_ok());
    assert_eq!(board.unwrap().get_board_state_hash(), 9595281602058382660)
}

#[test]
fn fen_string_parse_2() {
    let board = ChessBoard::from_forsyth_edwards("1r2k1r1/1p5p/2pp2pn/p1b1p3/2PnP1b1/NB1Q2p1/PP1P3q/R1B1K3 b - - 0 0".to_string());
    assert!(board.is_ok());
    assert_eq!(board.unwrap().get_board_state_hash(), 15171370747527475893)
}
