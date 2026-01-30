use crate::binary_mask::print_mask;

pub mod binary_mask;

struct ColorPieces {
    king: u64,
    queens: u64,
    rooks: u64,
    bishops: u64,
    knights: u64,
    pawns: u64,
}

struct ChessBoard {
    board: u64,
    player: u64,
    opponent: u64,

    player_pieces: ColorPieces,
    opponent_pieces: ColorPieces,

    is_white_to_play: bool,
    player_king_side_castle: bool,
    player_queen_side_castle: bool,
    opponent_king_side_castle: bool,
    opponent_queen_side_castle: bool,

    en_passant: u64,
}

fn get_starting_chessboard() -> ChessBoard {
    //return ChessBoard {
    //    board: (1 << 40) | (1 << 33) | (1 << 49),
    //    player: (1 << 40) | (1 << 33),
    //    opponent: (1 << 49),
    //    player_pieces: ColorPieces {
    //        king: 0,
    //        queens: 0,
    //        rooks: 0,
    //        bishops: 0,
    //        knights: 0,
    //        pawns: (1 << 40) | (1 << 33),
    //    },
    //    opponent_pieces: ColorPieces {
    //        king: 0,
    //        queens: 0,
    //        rooks: 0,
    //        bishops: 0,
    //        knights: 0,
    //        pawns: (1 << 49),
    //    },
    //    is_white_to_play: true,
    //    player_king_side_castle: true,
    //    player_queen_side_castle: true,
    //    opponent_king_side_castle: true,
    //    opponent_queen_side_castle: true,
    //    en_passant: (1 << 42),
    //};
    return ChessBoard {
        board: 18446462598732840960 | 65535,
        player: 18446462598732840960,
        opponent: 65535,
        player_pieces: ColorPieces {
            king: 1152921504606846976,
            queens: 576460752303423488,
            rooks: 9295429630892703744,
            bishops: 2594073385365405696,
            knights: 4755801206503243776,
            pawns: 71776119061217280,
        },
        opponent_pieces: ColorPieces {
            king: 16,
            queens: 8,
            rooks: 129,
            bishops: 36,
            knights: 66,
            pawns: 65280,
        },
        is_white_to_play: true,
        player_king_side_castle: true,
        player_queen_side_castle: true,
        opponent_king_side_castle: true,
        opponent_queen_side_castle: true,
        en_passant: 0,
    };
}

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
    fn get_fen(&self) -> String {
        // board
        let mut fen_board = String::new();
        for i in 0..64 {
            if self.player_pieces.king & (1 << i) != 0 {
                fen_board.push_str("K");
            } else if self.player_pieces.queens & (1 << i) != 0 {
                fen_board.push_str("Q");
            } else if self.player_pieces.rooks & (1 << i) != 0 {
                fen_board.push_str("R");
            } else if self.player_pieces.bishops & (1 << i) != 0 {
                fen_board.push_str("B");
            } else if self.player_pieces.knights & (1 << i) != 0 {
                fen_board.push_str("N");
            } else if self.player_pieces.pawns & (1 << i) != 0 {
                fen_board.push_str("P");
            } else if self.opponent_pieces.king & (1 << i) != 0 {
                fen_board.push_str("k");
            } else if self.opponent_pieces.queens & (1 << i) != 0 {
                fen_board.push_str("q");
            } else if self.opponent_pieces.rooks & (1 << i) != 0 {
                fen_board.push_str("r");
            } else if self.opponent_pieces.bishops & (1 << i) != 0 {
                fen_board.push_str("b");
            } else if self.opponent_pieces.knights & (1 << i) != 0 {
                fen_board.push_str("n");
            } else if self.opponent_pieces.pawns & (1 << i) != 0 {
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

    fn get_moves(&self, ma: binary_mask::MainHashtables) -> Vec<u16> {
        // TODO use something else than a vec
        let mut moves: Vec<u16> = vec![];
        let pieces = &self.player_pieces;
        let mut player_board = self.player;

        while player_board != 0 {
            // TODO optimize conditions
            let i = player_board.trailing_zeros() as u8;
            let index = 1 << i;
            if index & pieces.pawns != 0 {
                moves.append(&mut self.get_pawn_moves(i, &ma));
            } else if index & pieces.king != 0 {
                moves.append(&mut self.get_king_moves(i, &ma));
            } else if index & pieces.rooks != 0 {
                moves.append(&mut self.get_rook_moves(i, &ma));
            } else if index & pieces.bishops != 0 {
                moves.append(&mut self.get_bishop_moves(i, &ma));
            } else if index & pieces.knights != 0 {
                moves.append(&mut self.get_knight_moves(i, &ma));
            } else if index & pieces.queens != 0 {
                moves.append(&mut self.get_queen_moves(i, &ma));
            }
            player_board ^= index;
        }
        moves
    }
}

fn main() {
    let ma = binary_mask::generate_main_hashtables();
    let chessboard = get_starting_chessboard();
    print_mask(chessboard.player | chessboard.opponent);
    let moves = chessboard.get_moves(ma);
    println!("{:?}", moves);
}
