struct ChessBoard {
    white_king: u64,
    white_queen: u64,
    white_rook: u64,
    white_bishop: u64,
    white_knight: u64,
    white_pawn: u64,

    black_king: u64,
    black_queen: u64,
    black_rook: u64,
    black_bishop: u64,
    black_knight: u64,
    black_pawn: u64,

    is_white_to_play: bool,
    white_king_side_castle: bool,
    white_queen_side_castle: bool,
    black_king_side_castle: bool,
    black_queen_side_castle: bool,
}

fn get_starting_chessboard() -> ChessBoard {
    return ChessBoard {
        white_king: 1152921504606846976,
        white_queen: 576460752303423488,
        white_rook: 9295429630892703744,
        white_bishop: 2594073385365405696,
        white_knight: 4755801206503243776,
        white_pawn: 71776119061217280,
        black_king: 16,
        black_queen: 8,
        black_rook: 129,
        black_bishop: 36,
        black_knight: 66,
        black_pawn: 65280,
        is_white_to_play: true,
        white_king_side_castle: true,
        white_queen_side_castle: true,
        black_king_side_castle: true,
        black_queen_side_castle: true,
    };
}

impl ChessBoard {
    fn get_fen(&self) -> String {
        // board
        let mut fen_board = String::new();
        for i in 0..64 {
            if self.white_king & (1 << i) != 0 {
                fen_board.push_str("K");
            } else if self.white_queen & (1 << i) != 0 {
                fen_board.push_str("Q");
            } else if self.white_rook & (1 << i) != 0 {
                fen_board.push_str("R");
            } else if self.white_bishop & (1 << i) != 0 {
                fen_board.push_str("B");
            } else if self.white_knight & (1 << i) != 0 {
                fen_board.push_str("N");
            } else if self.white_pawn & (1 << i) != 0 {
                fen_board.push_str("P");
            } else if self.black_king & (1 << i) != 0 {
                fen_board.push_str("k");
            } else if self.black_queen & (1 << i) != 0 {
                fen_board.push_str("q");
            } else if self.black_rook & (1 << i) != 0 {
                fen_board.push_str("r");
            } else if self.black_bishop & (1 << i) != 0 {
                fen_board.push_str("b");
            } else if self.black_knight & (1 << i) != 0 {
                fen_board.push_str("n");
            } else if self.black_pawn & (1 << i) != 0 {
                fen_board.push_str("p");
            } else {
                fen_board.push_str("1");
            }
            if i % 8 == 7 {
                fen_board.push_str("/");
            }
        }
        fen_board = fen_board
            .replace("11111111", "8")
            .replace("1111111", "7")
            .replace("111111", "6")
            .replace("11111", "5")
            .replace("1111", "4")
            .replace("111", "3")
            .replace("11", "2");

        // player turn
        let fen_player_turn = String::from(if self.is_white_to_play { "w" } else { "b" });

        // castles
        let mut fen_castles = String::new();
        if self.white_king_side_castle {
            fen_castles.push_str("K");
        }
        if self.white_queen_side_castle {
            fen_castles.push_str("Q");
        }
        if self.black_king_side_castle {
            fen_castles.push_str("k");
        }
        if self.black_queen_side_castle {
            fen_castles.push_str("q");
        }

        fen_board + " " + &fen_player_turn + " " + &fen_castles
    }
}

fn main() {
    let chessboard = get_starting_chessboard();
    let fen = chessboard.get_fen();
    println!("{:?}", fen);
}
