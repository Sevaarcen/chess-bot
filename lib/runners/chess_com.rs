use std::time::Duration;

use crate::{gamelogic::{board::ChessBoard, index_pair_to_name, GameEnd, MoveType, Side}, stratagems::Stratagem};

use super::{Runner, RunnerError};

use thirtyfour_sync::http::reqwest_sync::ReqwestDriverSync;
use thirtyfour_sync::{prelude::*, GenericWebDriver};
use thirtyfour_sync::common::cookie::SameSite;
use serde_json::json;


pub struct ChessComGame {
    driver: GenericWebDriver<ReqwestDriverSync>,
    pub board: ChessBoard,
    player_side: Side,
    player_bot: Box<dyn Stratagem>,
    current_turn: Side,
    turn_number: usize
}


impl Runner for ChessComGame {
    fn initialize<T: Stratagem + 'static>(args: Vec<String>) -> Result<Self, RunnerError>
        where Self: Sized
    {
        if args.is_empty() {
            return Err(RunnerError::InitializationFaliure("You must provide a valid PHPSESSID value for this runner".to_string()));
        }

        let phpsessid = args[0].clone();

        // Start webdriver using: `geckodriver.exe --port 4444 --binary "C:\Program Files\Mozilla Firefox\firefox.exe"`
        let caps = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:4444", &caps).expect("Unable to connect to WebDriver");

        // navigate to chess.com and set the session id cookie to use pre-existing authentication
        driver.get("https://www.chess.com").unwrap();
        let mut cookie = Cookie::new("PHPSESSID", json!(phpsessid));
        cookie.set_domain(Some(".chess.com".to_string()));
        cookie.set_path(Some("/".to_string()));
        cookie.set_same_site(Some(SameSite::Lax));
        driver.add_cookie(cookie).unwrap();
        driver.refresh().unwrap();

        println!("\nNavigate to the chess game, and hit enter when ready to run the bot...\n");

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let chess_board_element = driver.find_element(By::Tag("chess-board")).expect("chess-board element doesn't exist -- are you in a game?");
        let cb_class_name = chess_board_element.class_name().unwrap().unwrap();

        let board = ChessBoard::new();

        let player_side = if cb_class_name == "board" {
            Side::White
        } else if cb_class_name == "board flipped" {
            Side::Black
        } else {
            panic!("Unable to determine player side from HTML... are you in a game?");
        };

        let player_bot = Box::new(<T as Stratagem>::initialize(player_side));

        Ok(Self {
            driver,
            board,
            player_side,
            player_bot,
            current_turn: Side::White,
            turn_number: 0  // start at 0 since we're using it as an offset
        })
    }


    fn run_game(self: &mut Self) -> Result<GameEnd, RunnerError> {
        loop {
            if let Some(v) = self.check_victory() {
                println!("\nGAME OVER: {:?}\n\nPress enter to exit...", v);
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                return Ok(v);
            }
            if self.current_turn == self.player_side {
                self.execute_bot_move().expect("Failed to perform bot move");
            } else {
                self.wait_for_player_turn();
                self.refresh_state().expect("Failed to refresh game state");
            }
            if self.current_turn == Side::Black {
                self.turn_number = self.turn_number + 1;
                eprintln!("We're now on turn {}", self.turn_number);
            }
        }
    }


    fn refresh_state(self: &mut Self) -> Result<(), RunnerError> {
        let chess_board_element = self.driver.find_element(By::Tag("chess-board")).unwrap();
        let highlighted_squares = chess_board_element.find_elements(By::ClassName("highlight")).unwrap();
        // it's empty at the start of the game and player has the first move
        // since no movies  as no moves have been performed and we don't need to update anything
        if highlighted_squares.len() != 2 {
            eprintln!("Refreshed state, there aren't two squares highlighted");
            return Ok(());
        }
        let square_1 = square_class_name_to_index_pair(highlighted_squares[1].class_name().unwrap().unwrap().as_str());
        let square_2 = square_class_name_to_index_pair(highlighted_squares[0].class_name().unwrap().unwrap().as_str());

        // figure out which is the FROM square and which is the TO
        let (from_square, to_square) = match self.board.get_square_by_index(square_1.0, square_1.1) {
            Some(piece) => {
                // if the square has an opponent piece, it must be the source square
                if piece.side == !self.player_side {
                    (square_1, square_2)
                } else {
                    (square_2, square_1)
                }
            },
            None => (square_2, square_1),
        };

        eprintln!("Opponent performed move {:?} to {:?}", index_pair_to_name(from_square.0, from_square.1).unwrap(), index_pair_to_name(to_square.0, to_square.1).unwrap());
        eprintln!("FEN after opponent move: {} (hash: {})", self.board.to_forsyth_edwards(), self.board.get_board_state_hash());
        let moved_piece = self.board.get_square_by_index(from_square.0, from_square.1).expect("Uhhh... the piece that's supposed to move doesn't exist");

        let the_move = moved_piece.get_specific_move(&self.board, to_square).expect("Uhhh... the move that the opponent performed isn't in the list of valid moves.");
        self.board.perform_move_and_record(&the_move).expect("Unable to perform opponent move");

        eprintln!("FEN after bot move: {} (hash: {})", self.board.to_forsyth_edwards(), self.board.get_board_state_hash());
        println!("{}", self.board);

        self.current_turn = !self.current_turn;
        Ok(())
    }


    fn execute_bot_move(self: &mut Self) -> Result<(), RunnerError> {
        let bot_move = self.player_bot.get_move(&self.board);
        let from_classname = index_pair_to_class_name(bot_move.from_square);
        let to_classname = index_pair_to_class_name(bot_move.destination);
        println!("Bot chose move from {} ({}) to {} ({})': {:#?}", index_pair_to_name(bot_move.from_square.0, bot_move.from_square.1).unwrap(), from_classname, index_pair_to_name(bot_move.destination.0, bot_move.destination.1).unwrap(), to_classname, bot_move);

        // with the chosen bot's move, perform it on chess.com and make sure it was actually performed.
        let from_square_element = self.driver.find_element(By::ClassName(&from_classname)).expect("Something went wrong -- unable to select FROM square");

        eprintln!("Clicking FROM square");
        self.driver.action_chain()
            .move_to_element_center(&from_square_element)
            .click()
            .perform()
            .expect("Unable to click FROM square");

        let to_square_element = self.driver.find_element(By::ClassName(&to_classname)).expect("Something went wrong -- unable to select TO square");

        eprintln!("Clicking TO square");
        self.driver.action_chain()
            .move_to_element_center(&to_square_element)
            .click()
            .perform()
            .expect("Unable to click TO square");

        // handle clicking the button to promote to queen
        if bot_move.move_type == MoveType::Promotion {
            eprintln!("Attempting promotion");
            // Since we're looking for an element with two CSS classes, use a . between the two classnames to select an element with both
            let classname = match self.player_side {
                Side::White => "promotion-piece.wq",
                Side::Black => "promotion-piece.bq",
            };
            let promotion_element = self.driver.find_element(By::ClassName(classname)).expect("No promotion view while attempting to promote!?");
            self.driver.action_chain()
                .move_to_element_center(&promotion_element)
                .click()
                .perform()
                .expect("Unable to click promotion button");
        }

        eprintln!("Done with bot interaction, recording move");
        self.board.perform_move_and_record(&bot_move).expect("Could not perform bot move");
        self.current_turn = !self.current_turn;

        println!("{}", self.board);

        Ok(()) // If we've gotten this far, no errors
    }


    fn check_victory(self: &Self) -> Option<GameEnd> {
       self.board.is_game_over(self.current_turn)
    }
}


impl ChessComGame {
    fn wait_for_player_turn(self: &Self) -> () {
        eprintln!("Waiting for player turn");
        // TODO re-evaluate wait duration -- may need to be quite a bit longer (5+ minutes)
        match self.player_side {
            Side::White => {
                let data_ply_num = self.turn_number * 2;
                if data_ply_num != 0 {
                    eprintln!("Waiting for data play number {}", data_ply_num);
                    self.driver.query(By::ClassName("black.node.selected")).with_attribute("data-ply", format!("{}", data_ply_num)).wait(Duration::from_secs(60), Duration::from_millis(250)).exists().unwrap();
                }
            },
            Side::Black => {
                let data_ply_num = self.turn_number * 2 + 1;
                eprintln!("Waiting for data play number {}", data_ply_num);
                self.driver.query(By::ClassName("white.node.selected")).with_attribute("data-ply", format!("{}", data_ply_num)).wait(Duration::from_secs(60), Duration::from_millis(250)).exists().unwrap();
            },
        };
        eprintln!("Finsihed waiting for player turn");
    }
}


fn square_class_name_to_index_pair(name: &str) -> (usize, usize) {
    let classname_split: Vec<&str> = name.splitn(2, '-').collect();
    let mut cell_iter = classname_split[1].chars().into_iter();
    let column = 7 + cell_iter.next().unwrap() as usize - '8' as usize;
    let row = 7 + cell_iter.next().unwrap() as usize - '8' as usize;
    let result = (column, row);
    eprintln!("Parsing class name '{}' into '{:?}'", name, result);
    result
}

fn index_pair_to_class_name(coordinates: (usize, usize)) -> String {
    format!("square-{}{}", coordinates.0 + 1, coordinates.1 + 1)
}