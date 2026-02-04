use crate::{ChessBoard, TypePiece};

impl ChessBoard {
    pub fn make_move(&mut self, move_code: u16) {
        // TODO implement: castling, en passant, promotion and sub-promotions
        let to_index = move_code & 0b111111;
        let to_index = to_index as usize;
        let from_index = (move_code >> 6) & 0b111111;
        let from_index = from_index as usize;
        let color = self.is_white_to_play as usize;
        let other_color = !self.is_white_to_play as usize;
        let move_xor = (1 << to_index) | (1 << from_index);
        let move_from_index = 1 << from_index;
        let move_to_index = 1 << to_index;

        // takes
        self.board |= move_to_index;
        self.pieces[self.pieces_by_index[to_index] as usize] &= !move_to_index;
        self.players[other_color] &= !move_to_index;

        self.pieces[self.pieces_by_index[from_index] as usize] ^= move_xor;
        self.board ^= move_from_index;
        self.players[color] ^= move_xor;

        self.pieces_by_index[to_index] = self.pieces_by_index[from_index];
        self.pieces_by_index[from_index] = TypePiece::Empty;

        self.is_white_to_play = !self.is_white_to_play;
    }
}
