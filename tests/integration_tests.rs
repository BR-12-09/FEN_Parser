/* Tests d'intégration pour le parseur FEN.

Ces tests vérifient le bon fonctionnement global du parseur avec des cas typiques et des cas limites. */

use fen_parser::parse_fen;
use fen_parser::types::{Color, Piece, PieceKind};

/* Test la position initiale standard */
#[test]
fn test_initial_position() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = parse_fen(fen).unwrap();

    assert_eq!(position.active_color, Color::White);
    assert_eq!(position.castling_rights.white_kingside, true);
    assert_eq!(position.castling_rights.white_queenside, true);
    assert_eq!(position.castling_rights.black_kingside, true);
    assert_eq!(position.castling_rights.black_queenside, true);
    assert_eq!(position.en_passant, None);
    assert_eq!(position.halfmove_clock, 0);
    assert_eq!(position.fullmove_number, 1);

    /* Vérification de quelques pièces clés */
    assert_eq!(
        position.pieces[0][0],
        Some(Piece {
            color: Color::White,
            kind: PieceKind::Rook
        })
    ); /* Tour a1 */
    assert_eq!(
        position.pieces[7][4],
        Some(Piece {
            color: Color::Black,
            kind: PieceKind::King
        })
    ); /* Roi e8 */
}

/* Test une position après plusieurs coups avec promotions */
#[test]
fn test_position_with_promotions() {
    let fen = "r1bq1bnr/ppPp1kpp/5n2/4p3/8/8/PPPP1PPP/RNBQKBNR w KQ - 1 10";
    let position = parse_fen(fen).unwrap();

    assert_eq!(position.active_color, Color::White);
    assert_eq!(
        position.pieces[1][2],
        Some(Piece {
            color: Color::White,
            kind: PieceKind::Pawn
        })
    ); /* Pion blanc en c7 */
    assert_eq!(position.halfmove_clock, 1);
    assert_eq!(position.fullmove_number, 10);
}

/* Test une position avec prise en passant possible */
#[test]
fn test_en_passant_position() {
    let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
    let position = parse_fen(fen).unwrap();

    assert_eq!(position.en_passant, Some((2, 5))); /* c6 */
    assert_eq!(position.halfmove_clock, 0);
}

/* Test une position sans aucun droit de roque */
#[test]
fn test_no_castling_rights() {
    let fen = "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b - - 3 3";
    let position = parse_fen(fen).unwrap();

    assert!(!position.castling_rights.has_any());
    assert_eq!(position.active_color, Color::Black);
}

/* Test une position avec roque uniquement côté roi noir */
#[test]
fn test_partial_castling_rights() {
    let fen = "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQk - 4 4";
    let position = parse_fen(fen).unwrap();

    assert!(position.castling_rights.white_kingside);
    assert!(position.castling_rights.white_queenside);
    assert!(position.castling_rights.black_kingside);
    assert!(!position.castling_rights.black_queenside); /* 'q' manque dans la FEN */
}

/* Test une FEN avec un nombre incorrect de rangées */
#[test]
fn test_invalid_row_count() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN w KQkq - 0 1", /* 7 rangées */
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8 w KQkq - 0 1", /* 9 rangées */
    ];

    for fen in fens {
        assert!(
            parse_fen(fen).is_err(),
            "Should have failed for FEN: {}",
            fen
        );
    }
}

/* Test des FEN avec des caractères invalides */
#[test]
fn test_invalid_piece_chars() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNX w KQkq - 0 1", /* 'X' invalide */
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e9 0 1", /* 'e9' invalide */
    ];

    for fen in fens {
        assert!(
            parse_fen(fen).is_err(),
            "Should have failed for FEN: {}",
            fen
        );
    }
}

/* Test des FEN avec des valeurs numériques invalides */
#[test]
fn test_invalid_numbers() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - a 1", /* demi-coups non numérique */
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 -1", /* tour négatif */
    ];

    for fen in fens {
        assert!(
            parse_fen(fen).is_err(),
            "Should have failed for FEN: {}",
            fen
        );
    }
}

/* Test des droits de roque invalides */
#[test]
fn test_invalid_castling() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQXkq - 0 1", /* 'X' invalide */
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkqk - 0 1", /* duplication */
    ];

    for fen in fens {
        assert!(
            parse_fen(fen).is_err(),
            "Should have failed for FEN: {}",
            fen
        );
    }
}

/* Test un plateau vide */
#[test]
fn test_empty_board() {
    let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
    let position = parse_fen(fen).unwrap();

    for rank in position.pieces.iter() {
        for square in rank.iter() {
            assert!(square.is_none());
        }
    }
}

/* Test une position avec le maximum de pièces */
#[test]
fn test_crowded_board() {
    let fen = "rnbqkbnr/pppppppp/PPPPPPPP/8/8/pppppppp/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = parse_fen(fen).unwrap();

    /* Vérifie que certaines cases attendues sont bien remplies */
    assert!(position.pieces[2][0].is_some()); /* Pion blanc en a6 */
    assert!(position.pieces[5][7].is_some()); /* Pion noir en h3 */
}

/* Test une position en milieu de partie avec compteurs avancés */
#[test]
fn test_midgame_position() {
    let fen = "r1bqkb1r/pp1p1ppp/2n1pn2/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 4 6";
    let position = parse_fen(fen).unwrap();

    assert_eq!(position.halfmove_clock, 4);
    assert_eq!(position.fullmove_number, 6);
    assert_eq!(position.active_color, Color::White);
    assert!(position.castling_rights.has_any());
}
