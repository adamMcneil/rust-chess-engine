#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fen_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        assert_eq!(board.board[0][0], Some(ChessPiece::Rook(Color::Black)));
        assert_eq!(board.board[7][7], Some(ChessPiece::Rook(Color::White)));
    }

    #[test]
    fn test_from_fen_custom_position() {
        let fen = "r1bqkb1r/pppp1ppp/2n5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 2 4";
        let board = Board::from_fen(fen);
        assert_eq!(board.board[0][0], Some(ChessPiece::Rook(Color::Black)));
        assert_eq!(board.board[7][7], Some(ChessPiece::Rook(Color::White)));
    }
}
