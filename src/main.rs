use chessbot_lib::{gamelogic::{pieces::Side, index_pair_to_name}, stratagems::{Stratagem, random_aggro::RandomAggro}, runners::{Connector, local_game::LocalGame, chess_com::ChessComGame}};

extern crate chessbot_lib;

use clap::{Parser, ValueEnum};


/// Semi-modular ChessBot for a ChessBot Tournament.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Bot strategy mode. Determines how the Bot's moves are chosen for a given board state.
    #[arg(value_enum, required=true)]
    strategem: StrategemChoices,

    /// Choice for how to interface the chess bot w/ a chess game. The runner handles reading state and giving the bot's inputs to the game.
    #[arg(value_enum, required=true)]
    runner: RunnerChoices,
}


#[derive(Debug, ValueEnum, Clone)]
#[value(rename_all="PascalCase")]
enum StrategemChoices {
    RandomAggro
}


#[derive(Debug, ValueEnum, Clone)]
#[value(rename_all="PascalCase")]
enum RunnerChoices {
    LocalGame
}


fn main() {
    let args = Args::parse();

    // let strategem_choice = match args.strategem {
    //     StrategemChoices::RandomAggro => RandomAggro
    // };

    let strategem = RandomAggro::initialize(Side::Black);
    //let mut local_game = LocalGame::initialize(Box::new(strategem)).unwrap();
    let mut game_runner = ChessComGame::initialize(Box::new(strategem)).unwrap();

    let mut bot_move = false;
    let victory = loop {
        if game_runner.check_victory().is_some() {
            break game_runner.check_victory().unwrap();
        }
        match bot_move {
            true => {
                game_runner.execute_bot_move().expect("Failed to perform bot move");
                bot_move = false; 
            },
            false => {
                game_runner.refresh_state().expect("Failed to refresh game state");
                bot_move = true;
            },
        }
    };
    println!("{}", "=".to_string().repeat(80));
    println!("{:?}", victory);
    println!("{}", "=".to_string().repeat(80));

}
