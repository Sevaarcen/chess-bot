use chessbot_lib::{stratagems::random_aggro::RandomAggro, runners::{Runner, local_game::LocalGame, chess_com::ChessComGame}};

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

    /// Arbitrary additional arguments as required by the different runners.
    #[arg(required=false)]
    runner_args: Vec<String>
}


#[derive(Debug, ValueEnum, Clone)]
#[value(rename_all="PascalCase")]
enum StrategemChoices {
    RandomAggro
}


#[derive(Debug, ValueEnum, Clone)]
#[value(rename_all="PascalCase")]
enum RunnerChoices {
    LocalGame,
    ChessCom
}


fn main() {
    let args = Args::parse();
    // eprintln!("{:#?}", args);

    // Given there's not a way to dynamically handle the type as a variable, instead we'll just handle each possible supported variation of runner+strategem combination.
    let mut game_runner: Box<dyn Runner> = match args.runner {
        RunnerChoices::LocalGame => match args.strategem {
            StrategemChoices::RandomAggro => Box::new(LocalGame::initialize::<RandomAggro>(args.runner_args).unwrap()),
        }
        RunnerChoices::ChessCom => match args.strategem {
            StrategemChoices::RandomAggro => Box::new(ChessComGame::initialize::<RandomAggro>(args.runner_args).unwrap()),
        }
    };

    let victory = game_runner.run_game().unwrap();
    println!("{}", "=".to_string().repeat(80));
    println!("{:?}", victory);
    println!("{}", "=".to_string().repeat(80));

}
