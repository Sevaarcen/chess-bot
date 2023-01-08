# chess-bot

A Chess Bot program entered into a friendly chess bot tournament.


# Strategems

Strategems dictate the strategy the chess bot uses throughout the game. Strategems have access to their own internal data, and the board state after every move.


__Available Strategems__
| Strategem     | Description
| ---           | ---
| RandomAggro   | Always takes a move that captures. If multiple captures are possible, the opponent piece with the highest material capture is taken, regardless of other factors. If no capture is available, a random move is taken. This Bot is unable to win via checkmate (unless it's by sheer luck), and is worse than Martin but less passive.


# Runners

Runners connect to an external source to get data on a chess game. Data is received on the Chess Bot opponent's move, and a Strategem of choice is used to perform automated moves in response.

__Available Runners__
| Runner        | Description
| ---           | ---
| LocalGame     | Runs a CLI game of chess where the external source is user input on the command line. Uses the Unicode chess characters and console coloring which may not work with all fonts nor terminals. VSCode terminal works well.
| ChessCom      | Uses your session token to open up and interact with chess.com using GeckoDriver.


### ChessCom
__Required Positional Arguments__
| Argument      | Type          | Description
| ---           | ---           | ---
| PHPSESSID     | String        | Your PHP session ID cookie taken from chess.com. Used for authentication for the web session.