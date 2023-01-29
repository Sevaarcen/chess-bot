use std::cmp::Ordering;

use itertools::Itertools;

use crate::gamelogic::{board::ChessBoard, pieces::{Side, PieceType}, ChessMove, name_to_index_pair, MoveType};

use super::Stratagem;

#[derive(Debug)]
enum GamePhase {
    Opening,
    MainGame
}

#[derive(Debug)]
struct PlannedMoveSequence {
    display_str: String,
    previous_moves: Vec<Option<ChessMove>>,
    planned_move: ChessMove
}

#[derive(Debug)]
struct DetailedMove {
    chess_move: ChessMove,
    is_hanging: bool,
    hangs_piece: bool,
    pre_num_threats: usize,
    post_num_threats: usize,
    pre_num_defends: usize,
    post_num_defends: usize,
    // pre_highest_threat: Option<usize>,
    // post_highest_threat:Option< usize>,
    pre_lowest_threatener: Option<usize>,
    post_lowest_threatener: Option<usize>,
    king_distance: usize
}


impl From<&str> for PlannedMoveSequence {
    fn from(s: &str) -> Self {
        let mut previous_moves = Vec::new();
        let parts = s.split(",").collect::<Vec<&str>>();
        for in_between in parts[0..parts.len()].iter() {
            match *in_between {
                "any" => {
                    previous_moves.push(None);
                },
                _ => {
                    let mut move_split = in_between.split("->");
                    let from_square = name_to_index_pair(move_split.next().unwrap().to_string()).unwrap();
                    let destination = name_to_index_pair(move_split.next().unwrap().to_string()).unwrap();
                    previous_moves.push(Some(ChessMove {
                        from_square,
                        destination,
                        move_type: MoveType::Standard, // This doesn't matter, so just fudge the values
                        captures: None,
                        dest_threatened: false,
                        dest_defended: false,
                    }));
                }
            }
        }
        
        let mut let_planned_move_str = parts.last().unwrap().split("->");
        let from_square = name_to_index_pair(let_planned_move_str.next().unwrap().to_string()).unwrap();
        let destination = name_to_index_pair(let_planned_move_str.next().unwrap().to_string()).unwrap();
        let planned_move = ChessMove {
            from_square,
            destination,
            move_type: MoveType::Standard, // This doesn't matter, so just fudge the values
            captures: None,
            dest_threatened: false,
            dest_defended: false,
        };
        Self {
            display_str: s.to_string(),
            previous_moves,
            planned_move
        }
    }
}


lazy_static! {
    static ref WHITE_PLANNED_OPENINGS: Vec<PlannedMoveSequence> = vec![
        PlannedMoveSequence::from("e2->e4")  // no previous moves
    ];
    static ref BLACK_PLANNED_OPENINGS:Vec<PlannedMoveSequence> = vec![
        PlannedMoveSequence::from("e2->e4,e7->e6,any,d8->f6"),
        PlannedMoveSequence::from("c2->c4,e7->e5"),
        PlannedMoveSequence::from("any,d7->d5,any,e7->e6")
    ];
}

pub struct ColeMiner {
    player_side: Side,
    current_state: GamePhase
}

impl Stratagem for ColeMiner {
    fn initialize(side: Side) -> Self where Self : Sized {
        println!("Cole Miner Strategem is active for side: {:?}", side);
        println!("Current phase: {:?}", GamePhase::Opening);
        match side {
            Side::White => println!("Planned Openings for White side: {:?}", WHITE_PLANNED_OPENINGS[0]),
            Side::Black => println!("Planned Openings for Black side: {:?}", BLACK_PLANNED_OPENINGS[0]),
        };
        ColeMiner { player_side: side, current_state: GamePhase::Opening }
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
        match self.player_side {
            Side::White => {
                for planned_sequence in WHITE_PLANNED_OPENINGS.iter() {
                    // the planned sequence must be shorter or equal to how many moves have occured, otherwise we're in uncharted territory
                    if planned_sequence.previous_moves.len() < board_state.move_list.len() {
                        break;
                    }
                    if std::iter::zip(&planned_sequence.previous_moves, &board_state.move_list).all(|(planned, actual)| planned.is_none() || planned.as_ref().unwrap() == actual) {
                        println!("All according to the plan: {}", planned_sequence.display_str);
                        preplanned_move = Some(planned_sequence.planned_move.clone());
                        break;
                    }
                }
            },
            Side::Black => {
                for planned_sequence in BLACK_PLANNED_OPENINGS.iter() {
                    if planned_sequence.previous_moves.len() < board_state.move_list.len() {
                        break;
                    }
                    if std::iter::zip(&planned_sequence.previous_moves, &board_state.move_list).all(|(planned, actual)| planned.is_none() || planned.as_ref().unwrap() == actual) {
                        println!("All according to the plan: {}", planned_sequence.display_str);
                        preplanned_move = Some(planned_sequence.planned_move.clone());
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
            let threats = board_state.get_square_threats(self.player_side, piece.position);
            let defends = board_state.get_square_threats(!self.player_side, piece.position);

            for m in piece_moves {
                let post_threats = board_state.get_square_threats(self.player_side, m.destination);
                let post_defends = board_state.get_square_threats(!self.player_side, m.destination);

                detailed_moves.push(DetailedMove {
                    chess_move: m.clone(),
                    is_hanging: !threats.is_empty() && defends.is_empty(),
                    hangs_piece: m.dest_threatened && !m.dest_defended,
                    pre_num_threats: threats.len(),
                    post_num_threats: post_threats.len(),
                    pre_num_defends: defends.len(),
                    post_num_defends: post_defends.len(),
                    // pre_highest_threat: threats.iter().map(|p| p.get_material()).sorted().last(),
                    // post_highest_threat: todo!(),
                    pre_lowest_threatener: threats.iter().map(|p| p.get_material()).sorted().last(),
                    post_lowest_threatener: post_threats.iter().map(|p| p.get_material()).sorted().last(),
                    king_distance: ((m.destination.0 as i64 - opponent_king.position.0 as i64).pow(2) + (m.destination.1 as i64 - opponent_king.position.1 as i64).pow(2)) as usize,
                })
            }
        }

        detailed_moves
    }

    fn rank_move(the_move: DetailedMove) -> usize {
        the_move.king_distance
    }

    fn get_standard_game_moves(self: &Self, board_state: &ChessBoard) -> ChessMove {
        let all_possible_moves = board_state.get_all_moves(self.player_side);
        let all_player_pieces = board_state.get_all_pieces(self.player_side);


        let best_cap = all_possible_moves.iter()
            // filter to only captures for our additional processing
            .filter(|m| m.captures.is_some())
            // Remove any captures that are an obvious blunder
            .filter(|m| {
                let player_piece = board_state.get_square_by_position(m.from_square).unwrap();
                let threatened_piece = board_state.get_square_by_position(m.captures.unwrap()).unwrap();

                return !m.dest_defended || player_piece.get_material() <= threatened_piece.get_material();
            })
            // Sort by the highest material capture
            .sorted_by(|m1, m2| {
                let m1_threatened_piece = board_state.get_square_by_position(m1.captures.unwrap()).unwrap();
                let m2_threatened_piece = board_state.get_square_by_position(m2.captures.unwrap()).unwrap();

                m2_threatened_piece.get_material().cmp(&m1_threatened_piece.get_material())
            })
            // and just take the first entry, we don't care about the 2nd best move (for now)
            .next();
        
        // get hanging pieces, we want to move them first if there's no better moves
        // TODO - account for the material value difference in what's threatening
        let hanging_pieces = all_player_pieces
            .iter()
            // filter to only threatened pieces
            .filter(|piece| {
                    board_state.is_square_threatened(self.player_side, piece.position)  // threatened
                    &&
                    !board_state.is_square_threatened(self.player_side, piece.position)  // not defended
            })
            .sorted_by(|cp1, cp2| {
                cp2.get_material().cmp(&cp1.get_material()) // reverse short
            })
            .collect_vec();
        
        if !hanging_pieces.is_empty() {
            println!("Pieces be hangin'");
            println!("{:#?}", hanging_pieces);
        }
        
        let non_capture_moves = all_possible_moves.iter()
            // filter to only non-capture moves
            .filter(|m| m.captures.is_none())
            .sorted_by(|m1, m2| {
                // first sort by if we're about to hang
                if (m1.dest_threatened && m1.dest_defended) && !(m2.dest_threatened && m2.dest_defended) {
                    return Ordering::Less;
                } else if (m2.dest_threatened && m2.dest_defended) && !(m1.dest_threatened && m1.dest_defended) {
                    return Ordering::Greater;
                }

                // whatever moves forward the most
                let opponent_side_row: usize = match self.player_side {
                    Side::White => 0,
                    Side::Black => 7,
                };
                let m1_dist = m1.destination.1.abs_diff(opponent_side_row);
                let m2_dist = m2.destination.1.abs_diff(opponent_side_row);
                m2_dist.cmp(&m1_dist)
            })
            .collect_vec();
        
        // figure out how to even transition to endgame

        // If there's a good capture, then perform it, otherwise perform another move that doesn't hang a piece
        if let Some(cap) = best_cap {
            return cap.clone();
        } else {
            non_capture_moves[0].clone()
        }
    }
}