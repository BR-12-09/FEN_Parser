/* Module de gestion des erreurs pour le parseur FEN.

Définit le type `FenError` et ses variantes pour représenter les différentes erreurs pouvant survenir lors du parsing FEN. */

use thiserror::Error;

/* Enumération des erreurs spécifiques au parsing FEN */
#[derive(Debug, Error)]
pub enum FenError {
    /* Erreur de format général de la chaîne FEN */
    #[error("Invalid FEN format: {0}")]
    InvalidFormat(String),

    /* Erreur dans la partie placement des pièces */
    #[error("Invalid piece placement: {0}")]
    InvalidPiecePlacement(String),

    /* Erreur dans la couleur active */
    #[error("Invalid active color: {0}")]
    InvalidActiveColor(String),

    /* Erreur dans les droits de roque */
    #[error("Invalid castling rights: {0}")]
    InvalidCastlingRights(String),

    /* Erreur dans la case de prise en passant */
    #[error("Invalid en passant square: {0}")]
    InvalidEnPassant(String),

    /* Erreur dans le compteur de demi-coups */
    #[error("Invalid halfmove clock: {0}")]
    InvalidHalfmoveClock(String),

    /* Erreur dans le numéro du tour */
    #[error("Invalid fullmove number: {0}")]
    InvalidFullmoveNumber(String),

    /* Erreur inconnue */
    #[error("Unknown parsing error")]
    Unknown,
}
