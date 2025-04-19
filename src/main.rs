/* Programme principal pour visualiser des positions FEN. */

use fen_parser::{ChessPosition, FenError};
use std::env;

fn main() -> Result<(), FenError> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <FEN_string>", args[0]);
        println!(
            "Example: {} \"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\"",
            args[0]
        );
        return Ok(());
    }

    let fen = &args[1];
    let position = ChessPosition::from_fen(fen)?;
    position.display_ascii();

    Ok(())
}
