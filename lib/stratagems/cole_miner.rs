use std::cmp::Ordering;

use itertools::Itertools;

use crate::gamelogic::{board::{ChessBoard, self}, pieces::{PieceType}, ChessMove, name_to_index_pair, MoveType, Side};

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

#[derive(Debug)]
struct DetailedMove {
    chess_move: ChessMove,
    is_hanging: bool,
    hangs_piece: bool,
    material_gain: i64,
    pre_num_threats: usize,
    post_num_threats: usize,
    pre_num_defends: usize,
    post_num_defends: usize,
    // pre_highest_threat: Option<usize>,
    // post_highest_threat:Option< usize>,
    pre_lowest_threatener: Option<usize>,
    post_lowest_threatener: Option<usize>,
    king_distance: usize,
    king_distance_change: usize
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
        PlannedMoveSequence::from("e2->e4")  // no previous moves
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
            let defends = board_state.get_square_threats(self.player_side, piece.position);

            for m in piece_moves {
                let post_threats = board_state.get_square_threats(!self.player_side, m.destination);
                let post_defends = board_state.get_square_threats(self.player_side, m.destination);

                detailed_moves.push(DetailedMove {
                    chess_move: m.clone(),
                    is_hanging: !threats.is_empty() && defends.is_empty(),
                    hangs_piece: !post_threats.is_empty() && post_defends.is_empty(),
                    material_gain: match m.captures {
                        Some(cap) => board_state.get_square_by_position(cap).unwrap().get_material() as i64 - piece.get_material() as i64,
                        None => 0
                    },
                    pre_num_threats: threats.len(),
                    post_num_threats: post_threats.len(),
                    pre_num_defends: defends.len(),
                    post_num_defends: post_defends.len(),
                    // pre_highest_threat: threats.iter().map(|p| p.get_material()).sorted().last(),
                    // post_highest_threat: todo!(),
                    pre_lowest_threatener: threats.iter().map(|p| p.get_material()).sorted().last(),
                    post_lowest_threatener: post_threats.iter().map(|p| p.get_material()).sorted().last(),
                    king_distance: get_distance(m.destination, opponent_king.position),
                    king_distance_change: get_distance(m.from_square, opponent_king.position).abs_diff(get_distance(m.destination, opponent_king.position))
                })
            }
        }

        detailed_moves
    }

    fn get_standard_game_moves(self: &Self, board_state: &ChessBoard) -> ChessMove {
        let all_possible_moves = self.get_detailed_moves(board_state);
        let ranked_moves = all_possible_moves.into_iter().sorted_by_key(|m| self.rank_move(m)).collect_vec();
        ranked_moves[ranked_moves.len() -1].chess_move.clone()
    }

    fn rank_move(self: &Self, the_move: &DetailedMove) -> i64 {
        let row_change = the_move.chess_move.from_square.1 as i64 - the_move.chess_move.destination.1 as i64;
        let num_towards_row = (7 - self.opponent_row as i64 - row_change.abs()).abs();

        // TODO actually figure out the ranking
        let score: f64 = (num_towards_row as f64 * 0.125)
                       + (the_move.king_distance_change as f64 * (1.0/64.0))
                       + (-100 * the_move.hangs_piece as i32) as f64  // very much discourage hanging pieces
                       + (100 * the_move.is_hanging as i32) as f64  // move pieces that are hanging first
                       + the_move.material_gain as f64 * 5.0;

        eprintln!("Scored from {:?} to {:?} as {}: {:?}", the_move.chess_move.from_square, the_move.chess_move.destination, score, the_move);
        (score * 100.0) as i64

    }
}

fn get_distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    ((pos1.0 as i64 - pos2.0 as i64).pow(2) as f64 + (pos1.1 as i64 - pos2.1 as i64).pow(2) as f64).powf(0.5) as usize
}