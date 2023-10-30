use super::base::{Board, Color};
use super::moves::exp_to_crd;

fn print_repr(arr: &[[char; 8]; 8]) {
    for n in (0..8).rev() {
        for k in 0..8 {
            print!("{}|", arr[k][n]);
        }
        print!("\n");
    }
}

macro_rules! draw_piece {
    ($repr:expr, $piece:expr, $sign:expr) => {
        for ptr in (0..64).map(|x| 2u64.pow(x)) {
            if ptr & $piece.pos != 0 {
                let crd = exp_to_crd(ptr);
                $repr[crd.0 as usize][crd.1 as usize] = $sign;
            }
        }
    };
}

fn repr_board(board: &Board) -> [[char; 8]; 8] {
    let mut arr = [[' '; 8]; 8];
    let player_w = match board.0.color {
        Color::White => board.0,
        Color::Black => board.1,
    };
    let player_b = match board.0.color {
        Color::White => board.1,
        Color::Black => board.0,
    };
    draw_piece!(arr, player_w.pawn, 'P');
    draw_piece!(arr, player_w.knight, 'H');
    draw_piece!(arr, player_w.bishop, 'B');
    draw_piece!(arr, player_w.rook, 'R');
    draw_piece!(arr, player_w.queen, 'Q');
    draw_piece!(arr, player_w.king, 'K');

    draw_piece!(arr, player_b.pawn, 'p');
    draw_piece!(arr, player_b.knight, 'h');
    draw_piece!(arr, player_b.bishop, 'b');
    draw_piece!(arr, player_b.rook, 'r');
    draw_piece!(arr, player_b.queen, 'q');
    draw_piece!(arr, player_b.king, 'k');
    arr
}

pub fn print_board(board: &Board) {
    print_repr(&repr_board(&board));
}
