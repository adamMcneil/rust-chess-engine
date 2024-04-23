
mod board;
use board::Board;    

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen = "r1bqkb1r/pppp1ppp/2n5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 2 4";
    let fen = "rn1qk2r/pbp2p1p/1p2pBp1/4P3/2B1p3/2N5/PPP2PPP/R2Q1RK1 b kq - 0 12";
    let board = Board::from_fen(fen);
    board.pretty_print();
}
