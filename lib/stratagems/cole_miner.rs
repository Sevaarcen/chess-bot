use itertools::Itertools;

use crate::gamelogic::{board::ChessBoard, pieces::PieceType, ChessMove, name_to_index_pair, MoveType, Side};

use super::Stratagem;

#[derive(Debug)]
enum GamePhase {
    Opening,
    MainGame
}

#[derive(Debug)]
struct PlannedMoveSequence {
    display_str: String,
    move_list: Vec<Option<ChessMove>>
}

#[allow(dead_code)]
#[derive(Debug)]
struct DetailedMove {
    chess_move: ChessMove,
    piece_type: PieceType,
    piece_materials: usize,
    is_hanging: bool,
    hangs_piece: bool,
    causes_check: bool,
    victory: bool,
    capture_materials: usize,
    total_hanging_materials: i64,
    pre_num_threats: usize,
    post_num_threats: usize,
    pre_num_defends: usize,
    post_num_defends: usize,
    pre_lowest_threatener: Option<usize>,
    post_lowest_threatener: Option<usize>,
    king_distance: usize,
    king_distance_change: i64,
}

impl From<&str> for PlannedMoveSequence {
    fn from(s: &str) -> Self {
        let mut move_list = Vec::new();
        let parts = s.split(",").collect::<Vec<&str>>();
        for planned in parts.iter() {
            match *planned {
                "any" => {
                    move_list.push(None);
                },
                _ => {
                    let mut move_split = planned.split("->");
                    let from_square = name_to_index_pair(move_split.next().unwrap().to_string()).unwrap();
                    let destination = name_to_index_pair(move_split.next().unwrap().to_string()).unwrap();
                    move_list.push(Some(ChessMove {
                        from_square,
                        destination,
                        move_type: MoveType::Standard, // This doesn't matter, so just fudge the values
                        captures: None
                    }));
                }
            }
        }
        Self {
            display_str: s.to_string(),
            move_list
        }
    }
}


lazy_static! {
    static ref WHITE_PLANNED_OPENINGS: Vec<PlannedMoveSequence> = vec![
        PlannedMoveSequence::from("e2->e4,e7->e5,c2->c3,any,d2->d4"),  // no previous moves
        PlannedMoveSequence::from("e2->e4,d7->d5,f2->f3"),
        PlannedMoveSequence::from("e2->e4,d7->d5,d2->d3,f5->e4,d3->e4,any,f2->f3"),
        PlannedMoveSequence::from("e2->e4,g8->f6,d2->d3"),
        PlannedMoveSequence::from("e2->e4,any,d1->e2"),
    ];
    static ref BLACK_PLANNED_OPENINGS:Vec<PlannedMoveSequence> = vec![
        PlannedMoveSequence::from("e2->e4,e7->e6,e4->e5,f7->f6"),
        PlannedMoveSequence::from("e2->e4,e7->e6,any,d8->f6"),
        PlannedMoveSequence::from("c2->c4,e7->e5"),
        PlannedMoveSequence::from("any,d7->d5,any,e7->e6")
    ];
}

pub struct ColeMiner {
    player_side: Side,
    current_state: GamePhase,
    opponent_row: usize
}

impl Stratagem for ColeMiner {
    fn initialize(side: Side) -> Self where Self : Sized {
        println!("Cole Miner Strategem is active for side: {:?}", side);
        println!("Current phase: {:?}", GamePhase::Opening);
        let opponent_row = match side {
            Side::White => {
                println!("Planned Openings for White side: {:?}", WHITE_PLANNED_OPENINGS[0]);
                0
            },
            Side::Black => {
                println!("Planned Openings for Black side: {:?}", BLACK_PLANNED_OPENINGS[0]);
                7
            },
        };
        ColeMiner { player_side: side, current_state: GamePhase::Opening , opponent_row}
    }

    fn get_move(self: &mut Self, board_state: &ChessBoard) -> ChessMove {
        match self.current_state {
            GamePhase::Opening => self.get_opening_moves(board_state),
            GamePhase::MainGame => self.get_standard_game_moves(board_state)
        }
    }
}

impl ColeMiner {
    fn get_opening_moves(self: &mut Self, board_state: &ChessBoard) -> ChessMove {
        // Figure out if the current moves of the game match one of the pre-generated move lists, and
        let mut preplanned_move: Option<ChessMove> = None;
        let num_moves_performed = board_state.move_list.len();
        match self.player_side {
            Side::White => {
                for planned_sequence in WHITE_PLANNED_OPENINGS.iter() {
                    // the planned sequence must be shorter or equal to how many moves have occured, otherwise we're in uncharted territory
                    if planned_sequence.move_list.len() < num_moves_performed {
                        break;
                    }
                    if std::iter::zip(&planned_sequence.move_list, &board_state.move_list).all(|(planned, actual)| planned.is_none() || planned.as_ref().unwrap() == actual) {
                        println!("All according to the plan: {}", planned_sequence.display_str);
                        preplanned_move = Some(planned_sequence.move_list[num_moves_performed].clone().unwrap());
                        break;
                    }
                }
            },
            Side::Black => {
                for planned_sequence in BLACK_PLANNED_OPENINGS.iter() {
                    if planned_sequence.move_list.len() < num_moves_performed {
                        break;
                    }
                    if std::iter::zip(&planned_sequence.move_list, &board_state.move_list).all(|(planned, actual)| planned.is_none() || planned.as_ref().unwrap() == actual) {
                        println!("All according to the plan: {}", planned_sequence.display_str);
                        eprintln!("{:#?}", planned_sequence);
                        preplanned_move = Some(planned_sequence.move_list[num_moves_performed].clone().unwrap());
                        break;
                    }
                }
            }
        }

        match preplanned_move {
            Some(m)=> m,
            None => {
                // if we don't have any moves left in the list, go into midgame
                self.enter_main_game();
                self.get_standard_game_moves(board_state)
            },
        }
    }

    fn enter_main_game(self: &mut Self) {
        self.current_state = GamePhase::MainGame;
        println!("#==============================================================================#");
        println!("|  WE'RE OUT OF THE OPENING NOW                                                |");
        println!("#==============================================================================#");
    }

    fn get_detailed_moves(self: &Self, board_state: &ChessBoard) -> Vec<DetailedMove> {
        let mut detailed_moves = Vec::new();

        let all_player_pieces = board_state.get_all_pieces(self.player_side);
        let opponent_pieces = board_state.get_all_pieces(!self.player_side);
        let opponent_king = opponent_pieces.iter().find(|p| p.piece_type == PieceType::King).unwrap();

        for piece in all_player_pieces {
            let piece_moves = piece.get_moves(board_state);
            let threats = board_state.get_square_threats(!self.player_side, piece.position);
            let defends = board_state.get_square_threats(self.player_side, piece.position); // This is the defends BEFORE the move, so it should always be 1

            for m in piece_moves {
                let mut eval_board = board_state.clone();
                eval_board.perform_move(&m).unwrap();
                let post_threats = eval_board.get_square_threats(!self.player_side, m.destination);
                let post_defends = eval_board.get_square_threats(self.player_side, m.destination);

                let pre_lowest_threatener = threats.iter().map(|p| p.get_material()).sorted().last();
                let post_lowest_threatener = post_threats.iter().map(|p| p.get_material()).sorted().last();

                let total_hanging_materials = board_state.get_all_pieces(self.player_side)
                    .iter()
                    .find( |piece| {
                        let p_threats = eval_board.get_square_threats(!self.player_side, piece.position);
                        let p_defends = eval_board.get_square_threats(self.player_side, piece.position);
                        if p_threats.is_empty() {
                            false
                        } else if p_defends.is_empty() {
                            true
                        } else {
                            p_threats.iter().map(|p| p.get_material()).sorted().last().unwrap() < piece.get_material()
                        }
                    })
                    .map(|piece| {
                        piece.get_material() as i64
                    })
                    .iter()
                    .sum::<i64>();

                let is_hanging = if threats.is_empty() {
                    false
                } else if defends.is_empty() {
                    true
                } else {
                    pre_lowest_threatener.unwrap() < piece.get_material()
                };

                let hangs_piece = if post_threats.is_empty() {
                    false  // not hanging if there's no threats
                } else if post_defends.is_empty() {
                    true  // hangs if there is at least 1 threat and no defenders
                } else {
                    post_lowest_threatener.unwrap() < piece.get_material()  // also hanging if the threatener is cheaper than what they're threatening
                };

                detailed_moves.push(DetailedMove {
                    chess_move: m.clone(),
                    piece_type: piece.piece_type,
                    piece_materials: piece.get_material(),
                    is_hanging,
                    hangs_piece,
                    causes_check: eval_board.is_checked(!self.player_side),
                    victory: eval_board.is_game_over(!self.player_side).is_some(),
                    capture_materials: match m.captures {
                        Some(cap) => board_state.get_square_by_position(cap).unwrap().get_material(),
                        None => 0
                    },
                    total_hanging_materials,
                    pre_num_threats: threats.len(),
                    post_num_threats: post_threats.len(),
                    pre_num_defends: defends.len(),
                    post_num_defends: post_defends.len(),
                    // pre_highest_threat: threats.iter().map(|p| p.get_material()).sorted().last(),
                    // post_highest_threat: todo!(),
                    pre_lowest_threatener,
                    post_lowest_threatener,
                    king_distance: get_distance(m.destination, opponent_king.position),
                    king_distance_change: get_distance(m.from_square, opponent_king.position) as i64 - get_distance(m.destination, opponent_king.position) as i64,
                })
            }
        }

        detailed_moves
    }

    fn get_standard_game_moves(self: &Self, board_state: &ChessBoard) -> ChessMove {
        let all_possible_moves = self.get_detailed_moves(board_state);
        let ranked_moves = all_possible_moves.into_iter().sorted_by_key(|m| self.rank_move(m, board_state)).collect_vec();
        let best_move = &ranked_moves[ranked_moves.len() -1];
        let bmr = self.rank_move(best_move, board_state);
        eprintln!("Best move ranked as {}: {:#?}", bmr, best_move);
        best_move.chess_move.clone()
    }

    fn rank_move(self: &Self, the_move: &DetailedMove, board_state: &ChessBoard) -> i64 {
        let row_change = the_move.chess_move.from_square.1 as i64 - the_move.chess_move.destination.1 as i64;
        let num_towards_row = 7 - self.opponent_row as i64 - row_change.abs();

        let last_move = board_state.move_list.iter().nth(board_state.move_list.len() - 2).unwrap();
        let is_undo_move = the_move.chess_move.from_square == last_move.destination;

        //let pre_threatened_mat_diff = the_move.pre_lowest_threatener.unwrap_or(the_move.piece_materials) as f64 - the_move.piece_materials as f64;
        let post_threatened_mat_diff = the_move.post_lowest_threatener.unwrap_or(the_move.piece_materials) as f64 - the_move.piece_materials as f64;

        let material_gain = if the_move.post_num_threats != 0 {
            the_move.capture_materials as i64 - the_move.piece_materials as i64
        } else {
            the_move.capture_materials as i64
        };

        let specific_move_bias = match the_move.chess_move.move_type {
            MoveType::DoubleAdvance => 0.25,
            MoveType::Castle => 2.50,
            MoveType::Promotion => 7.50,
            _ => 0.00
        };

        let specific_piece_bias = match the_move.piece_type {
            PieceType::Pawn => 0.00,
            PieceType::Rook => 0.60,
            PieceType::Knight => 0.50,
            PieceType::Bishop => 0.40,
            PieceType::Queen => 0.75,
            PieceType::King => -0.75  // Avoid moving the king for no reason
        };

        // If you're wondering where these numbers came from... I made them up and they're not based on any concrete methodology
        let score: f64 = (num_towards_row as f64 * 2.50)  // Encourage advancing towards opponent side of board
                       + (the_move.king_distance_change as f64 * 5.00)  // Encourage moving towards the king
                       + (material_gain as f64 * 100.00)  // Encourage moves that result in material advantage, discourage moves that result in material loss
                       + (the_move.capture_materials as f64 * 25.00)  // Encourage trades
                       + (the_move.total_hanging_materials as f64 * -15.00)  // Discourage leaving pieces hanging, even if not the active piece
                       + (the_move.post_num_threats as f64 * 4.50)  // Encourage threatening as much as possible
                       + (post_threatened_mat_diff * 5.50 * ((the_move.post_num_defends > 0) as i32) as f64)  // Encourage adding new threats, but don't discourage removing threats
                       + ((-30 * the_move.hangs_piece as i32) as f64 * the_move.piece_materials as f64)  // Discourage hanging pieces
                       + (25 * the_move.is_hanging as i32) as f64  // Encourage moving hanging pieces
                       + (-30 * is_undo_move as i32) as f64  // Discourage repetition
                       + (20 * (the_move.causes_check as i32)) as f64  // Encourage checking
                       + (999_999 * (the_move.victory as i32)) as f64  // Highly encourage winning... not rocket science here.
                       + specific_move_bias  // Encourage certain move types
                       + specific_piece_bias  // Encourage certain pieces to move over other types
                       + rand::random::<f64>();  // w/ random noise to prevent consistent repetition

        // eprintln!("[DEBUG] Score of {} for move {:?}", score, the_move);

        // Convert to i64 so we can order them...
        (score * 100.0) as i64
    }
}

fn get_distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    (
        (pos1.0 as i64 - pos2.0 as i64).pow(2) as f64
        +
        (pos1.1 as i64 - pos2.1 as i64).pow(2) as f64
    ).powf(0.5) as usize
}