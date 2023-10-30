use crate::utils::base::*;

const PAWN0: Piece = Piece {
    pos: 2u64.pow(8)
        + 2u64.pow(9)
        + 2u64.pow(10)
        + 2u64.pow(11)
        + 2u64.pow(12)
        + 2u64.pow(13)
        + 2u64.pow(14)
        + 2u64.pow(15),
    typ: PieceType::Pawn,
};

const BISHOP0: Piece = Piece {
    pos: 2u64.pow(2) + 2u64.pow(5),
    typ: PieceType::Bishop,
};

const KNIGHT0: Piece = Piece {
    pos: 2u64 + 2u64.pow(6),
    typ: PieceType::Knight,
};

const ROOK0: Piece = Piece {
    pos: 1u64 + 2u64.pow(7),
    typ: PieceType::Rook,
};

const QUEEN0: Piece = Piece {
    pos: 2u64.pow(3),
    typ: PieceType::Queen,
};

const KING0: Piece = Piece {
    pos: 2u64.pow(4),
    typ: PieceType::King,
};

const PAWN1: Piece = Piece {
    pos: 2u64.pow(48)
        + 2u64.pow(49)
        + 2u64.pow(50)
        + 2u64.pow(51)
        + 2u64.pow(52)
        + 2u64.pow(53)
        + 2u64.pow(54)
        + 2u64.pow(55),
    typ: PieceType::Pawn,
};

const BISHOP1: Piece = Piece {
    pos: 2u64.pow(58) + 2u64.pow(61),
    typ: PieceType::Bishop,
};

const KNIGHT1: Piece = Piece {
    pos: 2u64.pow(57) + 2u64.pow(62),
    typ: PieceType::Knight,
};

const ROOK1: Piece = Piece {
    pos: 2u64.pow(56) + 2u64.pow(63),
    typ: PieceType::Rook,
};

const QUEEN1: Piece = Piece {
    pos: 2u64.pow(59),
    typ: PieceType::Queen,
};

const KING1: Piece = Piece {
    pos: 2u64.pow(60),
    typ: PieceType::King,
};

fn _cbh(control: Color, value: Color) -> Color {
    match control {
        Color::White => value,
        Color::Black => match value {
            Color::White => Color::Black,
            Color::Black => Color::White,
        },
    }
}

pub fn create_board(clr: Color) -> Board {
    Board(
        Player {
            color: _cbh(clr, Color::White),
            pawn: PAWN0,
            knight: KNIGHT0,
            bishop: BISHOP0,
            rook: ROOK0,
            queen: QUEEN0,
            king: KING0,
        },
        Player {
            color: _cbh(clr, Color::Black),
            pawn: PAWN1,
            knight: KNIGHT1,
            bishop: BISHOP1,
            rook: ROOK1,
            queen: QUEEN1,
            king: KING1,
        },
    )
}
