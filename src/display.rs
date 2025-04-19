/* Affichage ASCII/Unicode d'une position d'échecs.

Ce module fournit des méthodes pour visualiser une position ChessPosition sous forme textuelle dans un terminal. */

use crate::types::{ChessPosition, PieceKind};

/* Affiche la position en ASCII/Unicode dans le terminal */
impl ChessPosition {
    pub fn display_ascii(&self) {
        println!("  +-----------------+");
        /* Convertit une pièce en caractère Unicode */
        for rank in (0..8).rev() {
            print!("{} | ", rank + 1);
            for file in 0..8 {
                let c = match self.pieces[rank as usize][file as usize] {
                    Some(piece) => match (piece.color, piece.kind) {
                        (_, PieceKind::King) => '♔',
                        (_, PieceKind::Queen) => '♕',
                        (_, PieceKind::Rook) => '♖',
                        (_, PieceKind::Bishop) => '♗',
                        (_, PieceKind::Knight) => '♘',
                        (_, PieceKind::Pawn) => '♙',
                    },
                    None => '·',
                };
                print!("{} ", c);
            }
            println!("|");
        }
        println!("  +-----------------+");
        println!("    a b c d e f g h");

        /* Formate la couleur active */
        println!(
            "\nActive color: {}",
            match self.active_color {
                crate::types::Color::White => "White",
                crate::types::Color::Black => "Black",
            }
        );

        /* Formate les droits de roque en chaîne */
        println!(
            "Castling rights: {}{}{}{}",
            if self.castling_rights.white_kingside {
                "K"
            } else {
                ""
            },
            if self.castling_rights.white_queenside {
                "Q"
            } else {
                ""
            },
            if self.castling_rights.black_kingside {
                "k"
            } else {
                ""
            },
            if self.castling_rights.black_queenside {
                "q"
            } else {
                ""
            },
        );

        /* Formate la case de prise en passant */
        println!(
            "En passant: {}",
            match self.en_passant {
                Some((file, rank)) => format!("{}{}", (b'a' + file) as char, rank + 1),
                None => "-".to_string(),
            }
        );

        println!("Halfmove clock: {}", self.halfmove_clock);
        println!("Fullmove number: {}", self.fullmove_number);
    }
}
