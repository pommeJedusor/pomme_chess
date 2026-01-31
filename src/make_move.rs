use crate::{ChessBoard, TypePiece};

impl ChessBoard {
    pub fn make_move(&mut self, move_code: u16) {
        // TODO implement: castling, en passant, promotion and sub-promotions
        let to_index = move_code & 0b111111;
        let to_index = to_index as usize;
        let from_index = (move_code >> 6) & 0b111111;
        let from_index = from_index as usize;

        // takes
        if self.pieces_by_index[to_index] != TypePiece::Empty {
            self.pieces[self.pieces_by_index[to_index] as usize] ^= 1 << to_index;
            // TODO remove shitty if-else
            if self.is_white_to_play {
                self.black ^= 1 << to_index;
            } else {
                self.white ^= 1 << to_index;
            }
        } else {
            self.board ^= 1 << to_index;
        }

        let move_xor = (1 << to_index) | (1 << from_index);
        self.pieces[self.pieces_by_index[from_index] as usize] ^= move_xor;
        self.board ^= 1 << from_index;
        // TODO remove shitty if-else
        if self.is_white_to_play {
            self.white ^= move_xor;
        } else {
            self.black ^= move_xor;
        }

        self.pieces_by_index[to_index] = self.pieces_by_index[from_index];
        self.pieces_by_index[from_index] = TypePiece::Empty;

        self.is_white_to_play = !self.is_white_to_play;
    }
}
