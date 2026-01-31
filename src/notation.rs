use crate::{ChessBoard, TypePiece};

fn index_to_square(index: usize) -> String {
    let mut result = String::new();
    let x = index % 8;
    let y = index / 8;

    result.push_str(["a", "b", "c", "d", "e", "f", "g", "h"][x]);
    result.push_str(["8", "7", "6", "5", "4", "3", "2", "1"][y]);

    result
}

pub fn get_notation_from_move(move_code: u16) -> String {
    let mut s = String::new();
    let to_index = move_code & 0b111111;
    let from_index = (move_code >> 6) & 0b111111;
    index_to_square(from_index as usize) + &index_to_square(to_index as usize)
}

impl ChessBoard {
    pub fn get_fen(&self) -> String {
        // board
        let letters = [
            "K", "Q", "R", "B", "N", "P", "k", "q", "r", "b", "n", "p", "1",
        ];
        let mut fen_board = String::new();
        for i in 0..64 {
            fen_board.push_str(letters[self.pieces_by_index[i] as usize]);
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
        if self.player_king_side_castle {
            fen_castles.push_str("K");
        }
        if self.player_queen_side_castle {
            fen_castles.push_str("Q");
        }
        if self.opponent_king_side_castle {
            fen_castles.push_str("k");
        }
        if self.opponent_queen_side_castle {
            fen_castles.push_str("q");
        }

        fen_board + " " + &fen_player_turn + " " + &fen_castles
    }
}
