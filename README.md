# chess-bot

A Chess Bot program entered into a friendly chess bot tournament.


# Strategems

Strategems dictate the strategy the chess bot uses throughout the game. Strategems have access to their own internal data, and the board state after every move.


__Available Strategems__
| Strategem     | Description
| ---           | ---
| RandomAggro   | Always takes a move that captures. If multiple captures are possible, the opponent piece with the highest material capture is taken, regardless of other factors. If no capture is available, a random move is taken.


# Runners

Runners connect to an external source to get data on a chess game. Data is received on the Chess Bot opponent's move, and a Strategem of choice is used to perform automated moves in response.

__Available Runners__
| Runner        | Description
| ---           | ---
| LocalGame     | Runs a CLI game of chess where the external source is user input on the command line. Uses the Unicode chess characters and console coloring which may not work with all fonts nor terminals. VSCode terminal works well.
