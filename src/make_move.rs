use crate::{ChessBoard, TypePiece};

// move code
// 2 first bits = type:
//      00  -> normal move
//      01  -> castling
//      10  -> promotion
//      11  -> en-passant?
//
// if type == normal move:
// just do the move, normally according to the last 12 bits
// if type == castling:
// do the move according to the last 12 bits, then move the rook according to the 3th->4th bits:
//      00 -> WHITE-KINGSIDE
//      01 -> WHITE-QUEENSIDE
//      10 -> BLACK-KINGSIDE
//      11 -> BLACK-QUEENSIDE
// if type == promotion:
// remove the pawn from_index, remove the piece at the to_index if there is one, create at the
// to_index a piece according to the 3th->4th bits:
//      00 -> QUEEN
//      01 -> ROOK
//      10 -> BISHOP
//      11 -> KNIGHT

const MAKE_MOVE_FUNCS: [for<'a> fn(&'a mut ChessBoard, u16); 4] = [
    ChessBoard::make_move_normal,
    ChessBoard::make_move_castling,
    ChessBoard::make_move_promotion,
    ChessBoard::make_move_normal,
];

impl ChessBoard {
    pub fn make_move(&mut self, move_code: u16) {
        // TODO implement: castling
        let type_move = move_code >> 14;
        MAKE_MOVE_FUNCS[type_move as usize](self, move_code);
    }

    fn make_move_normal(&mut self, move_code: u16) {
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

        // en-passant
        // TODO: optimize
        if self.pieces[TypePiece::WhitePawn as usize] & self.en_passant != 0 {
            let piece_to_remove = (0b111111110000000000000000 & self.en_passant) << 8;
            self.pieces[TypePiece::BlackPawn as usize] ^= piece_to_remove;
            self.players[other_color] ^= piece_to_remove;
            self.board ^= piece_to_remove;
            self.pieces_by_index[to_index + 8] = TypePiece::Empty;
        } else if self.pieces[TypePiece::BlackPawn as usize] & self.en_passant != 0 {
            let piece_to_remove =
                (0b111111110000000000000000000000000000000000000000 & self.en_passant) >> 8;
            self.pieces[TypePiece::WhitePawn as usize] ^= piece_to_remove;
            self.players[other_color] ^= piece_to_remove;
            self.board ^= piece_to_remove;
            self.pieces_by_index[to_index - 8] = TypePiece::Empty;
        }

        // update en-passant
        // TODO: optimize
        if self.pieces_by_index[to_index] == TypePiece::WhitePawn && to_index + 16 == from_index {
            self.en_passant = 1 << (to_index + 8);
        } else if self.pieces_by_index[to_index] == TypePiece::BlackPawn
            && to_index - 16 == from_index
        {
            self.en_passant = 1 << (to_index - 8);
        } else {
            self.en_passant = 0;
        }

        // update-castling
        self.king_side_castle[1] = !(self.king_side_castle[1] == false
            || to_index == 63
            || from_index == 63
            || from_index == 60);
        self.queen_side_castle[1] = !(self.queen_side_castle[1] == false
            || to_index == 56
            || from_index == 56
            || from_index == 60);
        self.king_side_castle[0] = !(self.king_side_castle[0] == false
            || to_index == 7
            || from_index == 7
            || from_index == 4);
        self.queen_side_castle[0] = !(self.queen_side_castle[0] == false
            || to_index == 0
            || from_index == 0
            || from_index == 4);

        self.is_white_to_play = !self.is_white_to_play;
    }

    fn make_move_promotion(&mut self, move_code: u16) {
        let to_index = move_code & 0b111111;
        let to_index = to_index as usize;
        let from_index = (move_code >> 6) & 0b111111;
        let from_index = from_index as usize;
        let color = self.is_white_to_play as usize;
        let other_color = !self.is_white_to_play as usize;
        let move_xor = (1 << to_index) | (1 << from_index);
        let move_from_index = 1 << from_index;
        let move_to_index = 1 << to_index;
        let to_promotion = (move_code >> 12) & 0b11;

        // takes
        self.board |= move_to_index;
        self.pieces[self.pieces_by_index[to_index] as usize] &= !move_to_index;
        self.players[other_color] &= !move_to_index;

        self.pieces[self.pieces_by_index[from_index] as usize] ^= move_from_index;
        self.board ^= move_from_index;
        self.players[color] ^= move_xor;

        // promotion
        let promotion_piece_type =
            self.pieces_by_index[from_index] as usize - 4 + to_promotion as usize;
        self.pieces[promotion_piece_type] ^= move_to_index;

        self.pieces_by_index[from_index] = TypePiece::Empty;

        // update en-passant
        self.en_passant = 0;

        // update-castling
        self.king_side_castle[1] = !(self.king_side_castle[1] == false || to_index == 63);
        self.queen_side_castle[1] = !(self.queen_side_castle[1] == false || to_index == 56);
        self.king_side_castle[0] = !(self.king_side_castle[0] == false || to_index == 7);
        self.queen_side_castle[0] = !(self.queen_side_castle[0] == false || to_index == 0);

        self.is_white_to_play = !self.is_white_to_play;
    }

    fn make_move_castling(&mut self, move_code: u16) {
        let (
            king_xor,
            rook_xor,
            king_from_index,
            king_to_index,
            rook_from_index,
            rook_to_index,
            color,
            king_type,
            rook_type,
        ) = match (move_code >> 12) & 0b11 {
            0b00 => (
                0b101000000000000000000000000000000000000000000000000000000000000,
                0b1010000000000000000000000000000000000000000000000000000000000000,
                60,
                62,
                63,
                61,
                1,
                TypePiece::WhiteKing,
                TypePiece::WhiteRook,
            ),
            0b01 => (
                0b1010000000000000000000000000000000000000000000000000000000000,
                0b100100000000000000000000000000000000000000000000000000000000,
                60,
                58,
                56,
                59,
                1,
                TypePiece::WhiteKing,
                TypePiece::WhiteRook,
            ),
            0b10 => (
                0b1010000,
                0b10100000,
                4,
                6,
                7,
                5,
                0,
                TypePiece::BlackKing,
                TypePiece::BlackRook,
            ),
            _ => (
                0b10100,
                0b1001,
                4,
                2,
                0,
                3,
                0,
                TypePiece::BlackKing,
                TypePiece::BlackRook,
            ),
        };

        self.pieces[king_type as usize] ^= king_xor;
        self.pieces[rook_type as usize] ^= rook_xor;

        self.players[color] ^= king_xor | rook_xor;

        self.board ^= king_xor | rook_xor;

        self.pieces_by_index[king_to_index] = king_type;
        self.pieces_by_index[rook_to_index] = rook_type;
        self.pieces_by_index[king_from_index] = TypePiece::Empty;
        self.pieces_by_index[rook_from_index] = TypePiece::Empty;

        self.king_side_castle[color] = false;
        self.queen_side_castle[color] = false;

        self.en_passant = 0;

        self.is_white_to_play = !self.is_white_to_play;
    }
}
