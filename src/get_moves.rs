use crate::ChessBoard;
use crate::binary_mask;
use crate::binary_mask::MainHashtables;

const MOVE_FUNC_BY_PIECE: [for<'a, 'b> fn(&'a ChessBoard, u8, &'b MainHashtables) -> Vec<u16>; 6] = [
    ChessBoard::get_king_moves,
    ChessBoard::get_queen_moves,
    ChessBoard::get_rook_moves,
    ChessBoard::get_bishop_moves,
    ChessBoard::get_knight_moves,
    ChessBoard::get_pawn_moves,
];

// u16: last 6 bits = to_index, 6 bits before that = from_index
fn move_mask_to_u16(from_index: u8, mut mask: u64) -> Vec<u16> {
    let from_index = from_index as u16;
    let from_index = from_index << 6;

    // TODO: optimize by using something else than a vector
    let mut moves = vec![];
    while mask != 0 {
        let to_index = mask.trailing_zeros() as u16;
        let move_u16 = to_index | from_index;
        moves.push(move_u16);
        mask ^= 1 << to_index;
    }

    moves
}

impl ChessBoard {
    fn get_rook_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> Vec<u16> {
        let square_mask = &ma.rook_moves_masks_magical_numbers[index as usize].mask;
        let magical_number = &ma.rook_moves_masks_magical_numbers[index as usize].magic_number;
        let hashkey = (self.board & square_mask).wrapping_mul(*magical_number) >> 48;
        let rook_moves = ma.rook_mask_blockers_hashmaps[index as usize][hashkey as usize].unwrap();
        move_mask_to_u16(index, rook_moves & self.player ^ rook_moves)
    }

    fn get_bishop_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> Vec<u16> {
        let square_mask = &ma.bishop_moves_masks_magical_numbers[index as usize].mask;
        let magical_number = &ma.bishop_moves_masks_magical_numbers[index as usize].magic_number;
        let hashkey = (self.board & square_mask).wrapping_mul(*magical_number) >> 48;
        let bishop_moves =
            ma.bishop_mask_blockers_hashmaps[index as usize][hashkey as usize].unwrap();
        move_mask_to_u16(index, bishop_moves & self.player ^ bishop_moves)
    }

    fn get_knight_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> Vec<u16> {
        let knight_moves = ma.knight_move_masks[index as usize];
        move_mask_to_u16(index, knight_moves & self.player ^ knight_moves)
    }

    fn get_queen_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> Vec<u16> {
        // TODO optimize specifically for the queen
        let mut queen_moves = self.get_bishop_moves(index, ma);
        queen_moves.append(&mut self.get_rook_moves(index, ma));
        queen_moves
    }

    fn get_pawn_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> Vec<u16> {
        let index = index as usize;
        let color = !self.is_white_to_play as usize;
        let pawn_takes =
            ma.pawn_mask_takes_hashmaps[color][index] & (self.opponent | self.en_passant);
        let pawn_blockers =
            ma.pawn_mask_blockers_hashmaps[color][index][0] & (self.opponent | self.player);
        let hashkey = (pawn_blockers >> ma.pawn_offsets[color][index][1])
            | (pawn_blockers >> ma.pawn_offsets[color][index][0]) & 0b11;
        let pawn_moves = ma.pawn_mask_blockers_hashmaps[color][index][hashkey as usize];
        let moves = pawn_moves | pawn_takes;
        move_mask_to_u16(index as u8, moves)
    }

    fn get_king_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> Vec<u16> {
        // TODO add castling
        let king_moves = ma.king_move_masks[index as usize];
        move_mask_to_u16(index as u8, king_moves & self.player ^ king_moves)
    }

    pub fn get_moves(&self, ma: binary_mask::MainHashtables) -> Vec<u16> {
        // TODO use something else than a vec
        let mut moves: Vec<u16> = vec![];
        let mut player_board = self.player;

        while player_board != 0 {
            let i = player_board.trailing_zeros() as usize;
            let index = 1 << i;
            moves.append(&mut MOVE_FUNC_BY_PIECE[self.pieces_by_index[i] as usize](
                self, i as u8, &ma,
            ));
            player_board ^= index;
        }
        moves
    }
}
