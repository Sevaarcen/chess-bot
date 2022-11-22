use chessbot_lib::gamelogic::{pieces::{ChessPiece, Side, PieceType}, board::ChessBoard};
use chessbot_lib::gamelogic::index_pair_to_name;

extern crate chessbot_lib;

fn main() {

    let default_board = chessbot_lib::gamelogic::board::ChessBoard::new();
    println!("{}", default_board);

    let piece = default_board.get_square_by_name("a1".to_string()).unwrap().unwrap();
    let moves = piece.get_moves(&default_board);
    println!("{:?}", moves);
    println!();

    // Slightly modified https://www.chess.com/puzzles/problem/691396
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

    let custom_board = ChessBoard {
        squares: custom_setup_squares,
        state: Default::default()
    };
    println!("{}", custom_board);

    let piece = custom_board.get_square_by_name("d5".to_string()).unwrap().unwrap();
    let moves = piece.get_moves(&custom_board);
    println!("d5 moves --> {:?}", moves.iter().map(|pair| index_pair_to_name(pair.0, pair.1).unwrap()).collect::<Vec<String>>());

    let piece = custom_board.get_square_by_name("e1".to_string()).unwrap().unwrap();
    let moves = piece.get_moves(&custom_board);
    println!("e1 moves --> {:?}", moves.iter().map(|pair| index_pair_to_name(pair.0, pair.1).unwrap()).collect::<Vec<String>>());

    let piece = custom_board.get_square_by_name("f3".to_string()).unwrap().unwrap();
    let moves = piece.get_moves(&custom_board);
    println!("f3 moves --> {:?}", moves.iter().map(|pair| index_pair_to_name(pair.0, pair.1).unwrap()).collect::<Vec<String>>());

    let piece = custom_board.get_square_by_name("e4".to_string()).unwrap().unwrap();
    let moves = piece.get_moves(&custom_board);
    println!("e4 moves --> {:?}", moves.iter().map(|pair| index_pair_to_name(pair.0, pair.1).unwrap()).collect::<Vec<String>>());

    let piece = custom_board.get_square_by_name("e1".to_string()).unwrap().unwrap();
    let moves = piece.get_moves(&custom_board);
    println!("e1 moves --> {:?}", moves.iter().map(|pair| index_pair_to_name(pair.0, pair.1).unwrap()).collect::<Vec<String>>());
}
