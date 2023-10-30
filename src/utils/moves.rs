use crate::utils::base::*;

pub fn flip_board(board: &Board) -> Board {
    Board(board.1, board.0)
}

pub fn crd_to_flat(x: (u64, u64)) -> u64 {
    x.0 + 8 * x.1
}

pub fn flat_to_crd(x: u64) -> (u64, u64) {
    (x % 8, x / 8) // (x axis, y axis) or (column, row)
}

pub fn exp_to_crd(x: u64) -> (u64, u64) {
    flat_to_crd(x.ilog2() as u64)
}

pub fn crd_to_exp(x: (u64, u64)) -> u64 {
    2u64.pow(crd_to_flat(x) as u32)
}

pub fn all_of_player(player: &Player) -> u64 {
    player.pawn.pos
        ^ player.knight.pos
        ^ player.bishop.pos
        ^ player.rook.pos
        ^ player.queen.pos
        ^ player.king.pos
}

pub fn occupied_by(x: u64, player: &Player) -> bool {
    (x & all_of_player(player)) != 0
}

macro_rules! create_move_function {
    ($func_name:ident, $test_expr:expr, $mov:expr) => {
        pub fn $func_name(piece: &Piece, board: &Board) -> Option<Piece> {
            let crd = exp_to_crd(piece.pos);
            if $test_expr(crd) {
                // don't jump off the world
                return None;
            }
            let x = $mov(piece.pos);
            if occupied_by(x, &board.0) {
                return None;
            }
            Some(Piece::new(x, piece.typ))
        }
    };
}

create_move_function!(one_up, |c: (u64, u64)| -> bool { c.1 == 7 }, |x| x << 8);
create_move_function!(one_down, |c: (u64, u64)| -> bool { c.1 == 0 }, |x| x >> 8);
create_move_function!(one_right, |c: (u64, u64)| -> bool { c.0 == 7 }, |x| x << 1);
create_move_function!(one_left, |c: (u64, u64)| -> bool { c.0 == 0 }, |x| x >> 1);
create_move_function!(
    one_up_right,
    |c: (u64, u64)| -> bool { (c.0 == 7) | (c.1 == 7) },
    |x| x << 9
);
create_move_function!(
    one_up_left,
    |c: (u64, u64)| -> bool { (c.0 == 0) | (c.1 == 7) },
    |x| x << 7
);
create_move_function!(
    one_down_right,
    |c: (u64, u64)| -> bool { (c.0 == 7) | (c.1 == 0) },
    |x| x >> 7
);
create_move_function!(
    one_down_left,
    |c: (u64, u64)| -> bool { (c.0 == 0) | (c.1 == 0) },
    |x| x >> 9
);

pub fn pawn_up(piece: &Piece, board: &Board) -> Option<Piece> {
    let p = one_up(piece, board);
    match p {
        Some(x) => {
            if occupied_by(x.pos, &board.1) {
                return None;
            } else {
                return p;
            }
        }
        None => None,
    }
}

pub fn pawn_up_right(piece: &Piece, board: &Board) -> Option<Piece> {
    let p = one_up_right(piece, board);
    match p {
        Some(x) => {
            if occupied_by(x.pos, &board.1) {
                return p;
            } else {
                return None;
            }
        }
        None => None,
    }
}

pub fn pawn_up_left(piece: &Piece, board: &Board) -> Option<Piece> {
    let p = one_up_left(piece, board);
    match p {
        Some(x) => {
            if occupied_by(x.pos, &board.1) {
                return p;
            } else {
                return None;
            }
        }
        None => None,
    }
}

pub fn n_times(
    piece: &Piece,
    board: &Board,
    mov: fn(&Piece, &Board) -> Option<Piece>,
    n: u8,
) -> Option<Piece> {
    let m = mov(piece, board);
    if n == 1 {
        return m;
    }
    match m {
        None => None,
        Some(x) => n_times(&x, board, mov, n - 1),
    }
}

macro_rules! create_knight_move {
    ($func_name:ident, $bad_position:expr, $mov:expr) => {
        pub fn $func_name(piece: &Piece, board: &Board) -> Option<Piece> {
            let crd = exp_to_crd(piece.pos);
            if $bad_position(crd) {
                return None; // don't jump off the edge of the world
            }
            let x = $mov(piece.pos);
            if occupied_by(x, &board.0) {
                return None;
            }
            Some(Piece::new(x, piece.typ))
        }
    };
}

create_knight_move!(
    // further up
    knight_up_right,
    |c: (u64, u64)| -> bool { (c.0 == 7) | (c.1 >= 6) },
    |x| x << 17
);
create_knight_move!(
    // further up
    knight_up_left,
    |c: (u64, u64)| -> bool { (c.0 == 0) | (c.1 >= 6) },
    |x| x << 15
);
create_knight_move!(
    // further down
    knight_down_right,
    |c: (u64, u64)| -> bool { (c.0 == 7) | (c.1 <= 1) },
    |x| x >> 15
);
create_knight_move!(
    // further down
    knight_down_left,
    |c: (u64, u64)| -> bool { (c.0 == 0) | (c.1 <= 1) },
    |x| x >> 17
);
create_knight_move!(
    // further right
    knight_right_up,
    |c: (u64, u64)| -> bool { (c.0 >= 6) | (c.1 == 7) },
    |x| x << 10
);
create_knight_move!(
    // further left
    knight_left_up,
    |c: (u64, u64)| -> bool { (c.0 <= 1) | (c.1 == 7) },
    |x| x << 6
);
create_knight_move!(
    // further right
    knight_right_down,
    |c: (u64, u64)| -> bool { (c.0 >= 6) | (c.1 == 0) },
    |x| x >> 6
);
create_knight_move!(
    // further left
    knight_left_down,
    |c: (u64, u64)| -> bool { (c.0 <= 1) | (c.1 == 0) },
    |x| x >> 10
);
