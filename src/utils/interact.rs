use super::base::*;

pub fn parse_type(s: &String) -> PieceType {
    match s.as_str() {
        "bishop" => PieceType::Bishop,
        "knight" => PieceType::Knight,
        "rook" => PieceType::Rook,
        "queen" => PieceType::Queen,
        "king" => PieceType::King,
        _ => PieceType::Pawn,
    }
}
