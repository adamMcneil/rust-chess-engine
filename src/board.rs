pub(crate) struct Board {
    board: [[Option<ChessPiece>;8]; 8]
}

impl Board {
    pub fn new() -> Self {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Self::from_fen(fen)
    }

    pub fn pretty_print(&self) {
        println!("    a   b   c   d   e   f   g   h");
        println!(" ---------------------------------");
        for rank in (0..8).rev() {
            print!("{}|", rank + 1);
            for file in 0..8 {
                match self.board[rank][file] {
                    Some(piece) => {
                        let symbol = match piece {
                            ChessPiece::Pawn(Color::Black) => '♙',
                            ChessPiece::Pawn(Color::White) => '♟',
                            ChessPiece::Knight(Color::Black) => '♘',
                            ChessPiece::Knight(Color::White) => '♞',
                            ChessPiece::Bishop(Color::Black) => '♗',
                            ChessPiece::Bishop(Color::White) => '♝',
                            ChessPiece::Rook(Color::Black) => '♖',
                            ChessPiece::Rook(Color::White) => '♜',
                            ChessPiece::Queen(Color::Black) => '♕',
                            ChessPiece::Queen(Color::White) => '♛',
                            ChessPiece::King(Color::Black) => '♔',
                            ChessPiece::King(Color::White) => '♚',
                        };
                        print!(" {} |", symbol);
                    }
                    None => print!("   |"),
                }
            }
            println!();
            println!(" ---------------------------------");
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        const EMPTY: Option<ChessPiece> = None;
        let mut board = [[EMPTY; 8]; 8];
        let mut rank = 7;
        let mut file = 0;
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();

        for c in fen_parts[0].chars() {
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    let empty_squares = c.to_digit(10).unwrap() as usize;
                    for _ in 0..empty_squares {
                        board[rank][file] = None;
                        file += 1;
                    }
                } 
                'p' => {
                    board[rank][file] = Some(ChessPiece::Pawn(Color::Black));
                    file += 1;
                }
                'r' => {
                    board[rank][file] = Some(ChessPiece::Rook(Color::Black));
                    file += 1;
                }
                'n' => {
                    board[rank][file] = Some(ChessPiece::Knight(Color::Black));
                    file += 1;
                }
                'b' => {
                    board[rank][file] = Some(ChessPiece::Bishop(Color::Black));
                    file += 1;
                }
                'q' => {
                    board[rank][file] = Some(ChessPiece::Queen(Color::Black));
                    file += 1;
                }
                'k' => {
                    board[rank][file] = Some(ChessPiece::King(Color::Black));
                    file += 1;
                }
                'P' => {
                    board[rank][file] = Some(ChessPiece::Pawn(Color::White));
                    file += 1;
                }
                'R' => {
                    board[rank][file] = Some(ChessPiece::Rook(Color::White));
                    file += 1;
                }
                'N' => {
                    board[rank][file] = Some(ChessPiece::Knight(Color::White));
                    file += 1;
                }
                'B' => {
                    board[rank][file] = Some(ChessPiece::Bishop(Color::White));
                    file += 1;
                }
                'Q' => {
                    board[rank][file] = Some(ChessPiece::Queen(Color::White));
                    file += 1;
                }
                'K' => {
                    board[rank][file] = Some(ChessPiece::King(Color::White));
                    file += 1;
                }
                _ => {}
            }
        }

        Board { board }
    }
}

#[derive(Debug, Clone, Copy)]
enum ChessPiece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[derive(Debug, Clone, Copy)]
enum Color {
    White,
    Black,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

#[allow(dead_code)]
impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Move {
    from: Position,
    to: Position,
}

#[allow(dead_code)]
impl Move {
    fn new(from: Position, to: Position) -> Self {
        Move { from, to }
    }
}
