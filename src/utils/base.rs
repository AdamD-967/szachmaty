#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub pos: u64,
    pub typ: PieceType,
}

impl Piece {
    pub fn new(pos: u64, typ: PieceType) -> Piece {
        Piece { pos, typ }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Copy)]
pub struct Player {
    pub color: Color,
    pub pawn: Piece,
    pub knight: Piece,
    pub bishop: Piece,
    pub rook: Piece,
    pub queen: Piece,
    pub king: Piece,
}

#[derive(Clone, Copy)]
pub struct Board(pub Player, pub Player);
