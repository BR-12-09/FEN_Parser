/* Parseur FEN utilisant la crate Nom.

Ce module implémente le parsing d'une chaîne FEN selon la spécification standard. Il transforme une chaîne FEN en une structure ChessPosition. */

use crate::error::FenError;
use crate::types::{CastlingRights, ChessPosition, Color, Piece, PieceKind};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, one_of, space1},
    combinator::{map_res, opt, recognize},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};

/* Parse une chaîne FEN complète en structure ChessPosition */
pub fn parse_fen(fen: &str) -> Result<ChessPosition, FenError> {
    let (_, (pieces, active_color, castling, en_passant, halfmove, fullmove)) = tuple((
        parse_piece_placement,
        parse_active_color,
        parse_castling,
        parse_en_passant,
        parse_number,
        parse_number,
    ))(fen)
    .map_err(|_| FenError::InvalidFormat("Failed to parse FEN string".into()))?;

    Ok(ChessPosition {
        pieces,
        active_color,
        castling_rights: castling,
        en_passant,
        halfmove_clock: halfmove,
        fullmove_number: fullmove,
    })
}

/* Parse le placement des pièces (1ère partie du FEN) */
fn parse_piece_placement(input: &str) -> IResult<&str, [[Option<Piece>; 8]; 8]> {
    let (input, ranks) = separated_list1(tag("/"), parse_rank)(input)?;
    let (input, _) = space1(input)?;

    if ranks.len() != 8 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::LengthValue,
        )));
    }

    let mut board = [[None; 8]; 8];
    for (i, rank) in ranks.iter().enumerate() {
        if rank.len() != 8 {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::LengthValue,
            )));
        }
        board[7 - i] = *rank;
    }

    Ok((input, board))
}

/* Parse un seul rang du plateau */
fn parse_rank(input: &str) -> IResult<&str, [Option<Piece>; 8]> {
    let (input, pieces) = many1(alt((parse_piece, parse_empty)))(input)?;

    let mut rank = [None; 8];
    let mut idx = 0;

    for item in pieces {
        match item {
            RankItem::Piece(p) => {
                if idx >= 8 {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::LengthValue,
                    )));
                }
                rank[idx] = Some(p);
                idx += 1;
            }
            RankItem::Empty(count) => {
                idx += count as usize;
                if idx > 8 {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::LengthValue,
                    )));
                }
            }
        }
    }

    if idx != 8 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::LengthValue,
        )));
    }

    Ok((input, rank))
}

/* Types internes pour le parsing */
enum RankItem {
    Piece(Piece),
    Empty(u8),
}

/* Parse une pièce individuelle */
fn parse_piece(input: &str) -> IResult<&str, RankItem> {
    let (input, c) = one_of("KQRBNPkqrbnp")(input)?;
    let piece = match c {
        'K' => Piece {
            color: Color::White,
            kind: PieceKind::King,
        },
        'Q' => Piece {
            color: Color::White,
            kind: PieceKind::Queen,
        },
        'R' => Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        },
        'B' => Piece {
            color: Color::White,
            kind: PieceKind::Bishop,
        },
        'N' => Piece {
            color: Color::White,
            kind: PieceKind::Knight,
        },
        'P' => Piece {
            color: Color::White,
            kind: PieceKind::Pawn,
        },
        'k' => Piece {
            color: Color::Black,
            kind: PieceKind::King,
        },
        'q' => Piece {
            color: Color::Black,
            kind: PieceKind::Queen,
        },
        'r' => Piece {
            color: Color::Black,
            kind: PieceKind::Rook,
        },
        'b' => Piece {
            color: Color::Black,
            kind: PieceKind::Bishop,
        },
        'n' => Piece {
            color: Color::Black,
            kind: PieceKind::Knight,
        },
        'p' => Piece {
            color: Color::Black,
            kind: PieceKind::Pawn,
        },
        _ => unreachable!(),
    };
    Ok((input, RankItem::Piece(piece)))
}

/* Parse une série de cases vides (chiffre 1-8) */
fn parse_empty(input: &str) -> IResult<&str, RankItem> {
    let (input, count) = map_res(digit1, |s: &str| s.parse::<u8>())(input)?;
    Ok((input, RankItem::Empty(count)))
}

/* Parse la couleur active (w/b) */
fn parse_active_color(input: &str) -> IResult<&str, Color> {
    let (input, c) = one_of("wb")(input)?;
    let (input, _) = space1(input)?;
    let color = match c {
        'w' => Color::White,
        'b' => Color::Black,
        _ => unreachable!(),
    };
    Ok((input, color))
}

/* Parse les droits de roque */
fn parse_castling(input: &str) -> IResult<&str, CastlingRights> {
    let (input, s) =
        take_while1(|c: char| c == '-' || c == 'K' || c == 'Q' || c == 'k' || c == 'q')(input)?;
    let (input, _) = space1(input)?;

    let mut rights = CastlingRights::none();

    if s == "-" {
        return Ok((input, rights));
    }

    /* Vérifier les doublons */
    let mut seen = std::collections::HashSet::new();
    for c in s.chars() {
        if !seen.insert(c) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
        match c {
            'K' => rights.white_kingside = true,
            'Q' => rights.white_queenside = true,
            'k' => rights.black_kingside = true,
            'q' => rights.black_queenside = true,
            _ => (),
        }
    }

    Ok((input, rights))
}

/* Parse la case de prise en passant */
fn parse_en_passant(input: &str) -> IResult<&str, Option<(u8, u8)>> {
    let (input, ep) = alt((
        tag("-"),
        recognize(separated_pair(one_of("abcdefgh"), one_of("36"), tag(""))),
    ))(input)?;
    let (input, _) = space1(input)?;

    if ep == "-" {
        return Ok((input, None));
    }

    let file = ep.chars().next().unwrap() as u8 - b'a';
    let rank = ep.chars().nth(1).unwrap().to_digit(10).unwrap() as u8 - 1;

    Ok((input, Some((file, rank))))
}

/* Parse un nombre (pour demi-coups ou numéro de tour) */
fn parse_number(input: &str) -> IResult<&str, u32> {
    let (input, num) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = opt(space1)(input)?;
    Ok((input, num))
}
