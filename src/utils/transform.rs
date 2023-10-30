use super::base::*;

fn opu(piece: &Piece, from: &Piece, to: &Piece) -> Piece {
    // own piece updater
    if piece.typ == from.typ {
        Piece::new(piece.pos - from.pos + to.pos, piece.typ)
    } else {
        piece.clone()
    }
}

fn take(piece: &Piece, by: &Piece) -> Piece {
    Piece::new(piece.pos & !by.pos, piece.typ)
}

pub fn update_board(board: &Board, w_player: u8, from: &Piece, to: &Piece) -> Board {
    let mut p0: Player;
    let mut p1: Player;
    if w_player == 0 {
        p0 = board.0;
        p1 = board.1;
    } else {
        p0 = board.1;
        p1 = board.0;
    }
    p0 = Player {
        color: p0.color,
        pawn: opu(&p0.pawn, &from, &to),
        knight: opu(&p0.knight, &from, &to),
        bishop: opu(&p0.bishop, &from, &to),
        rook: opu(&p0.rook, &from, &to),
        queen: opu(&p0.queen, &from, &to),
        king: opu(&p0.king, &from, &to),
    };
    p1 = Player {
        color: p1.color,
        pawn: take(&p1.pawn, &to),
        knight: take(&p1.knight, &to),
        bishop: take(&p1.bishop, &to),
        rook: take(&p1.rook, &to),
        queen: take(&p1.queen, &to),
        king: take(&p1.king, &to),
    };
    if w_player == 1 {
        Board(p1, p0)
    } else {
        Board(p0, p1)
    }
}
