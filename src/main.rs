#![allow(dead_code)]
#![allow(unused_imports)]

mod utils;
use crate::utils::base::*;
use crate::utils::interact::*;
use crate::utils::moves::*;
use crate::utils::printer::*;
use crate::utils::starter::*;
use crate::utils::transform::update_board;

use text_io::read;

fn main() {
    println!("color: ");
    let mut inp: String = read!();
    let mut cnt: u8;

    let mut crd0: (u64, u64) = (0, 0);
    let mut crd1: (u64, u64) = (0, 0);

    let mut board: Board;
    let mut archiv: Board;
    if inp == "white" {
        board = create_board(Color::White);
        cnt = 0;
    } else {
        board = create_board(Color::Black);
        cnt = 1;
    }
    archiv = board;

    loop {
        print_board(&board);
        println!("what to do? ");
        inp = read!();
        if inp == "fuck" {
            board = archiv;
            continue;
        }
        if inp == "move" {
            archiv = board;
            println!("wut? ");
            inp = read!();
            println!("from: ");
            crd0.0 = read!();
            crd0.1 = read!();
            println!("to: ");
            crd1.0 = read!();
            crd1.1 = read!();
            board = update_board(
                &board,
                cnt,
                &Piece::new(crd_to_exp(crd0), parse_type(&inp)),
                &Piece::new(crd_to_exp(crd1), parse_type(&inp)),
            );
            continue;
        }
        if inp == "exit" {
            return;
        }
        cnt = (cnt + 1) % 2;
    }
}
