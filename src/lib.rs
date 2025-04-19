pub mod display;
pub mod error;
pub mod parser;
pub mod types;

// Ré-exporter les types principaux
pub use error::FenError;
pub use parser::parse_fen;
pub use types::{CastlingRights, ChessPosition, Color, Piece, PieceKind};
