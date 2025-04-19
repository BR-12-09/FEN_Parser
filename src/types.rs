/* Définitions des types fondamentaux pour représenter une position d'échecs.

Ce module contient les structures et enumerations nécessaires pour représenter une position d'échecs selon la notation FEN. */

use crate::error::FenError;
use crate::parser::parse_fen;

/* Couleur d'une pièce (Blanc ou Noir) */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

/* Type de pièce d'échecs */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

/* Pièce d'échecs avec sa couleur et son type */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

/* Droits de roque pour les deux camps */
#[derive(Debug, PartialEq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    /* Vérifie si au moins un droit de roque est disponible */
    pub fn has_any(&self) -> bool {
        self.white_kingside || self.white_queenside || self.black_kingside || self.black_queenside
    }

    /* Crée une instance sans aucun droit de roque */
    pub fn none() -> Self {
        Self {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }
}

/* Position complète d'un jeu d'échecs */
#[derive(Debug, PartialEq)]
pub struct ChessPosition {
    /* Plateau 8x8 représentant les pièces */
    pub pieces: [[Option<Piece>; 8]; 8],
    /* Camp ayant le trait */
    pub active_color: Color,
    /* Droits de roque disponibles */
    pub castling_rights: CastlingRights,
    /* Case de prise en passant */
    pub en_passant: Option<(u8, u8)>,
    /* Nombre de demi-coups depuis la dernière capture ou avance de pion */
    pub halfmove_clock: u32,
    /* Numéro du tour actuel */
    pub fullmove_number: u32,
}

/* Crée une position initiale standard */
impl Default for ChessPosition {
    fn default() -> Self {
        // On utilise directement la construction manuelle pour éviter la dépendance circulaire
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        parse_fen(fen).expect("Default FEN should be valid")
    }
}

/* Crée une position à partir d'une chaîne FEN */
impl ChessPosition {
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        parse_fen(fen)
    }
}
