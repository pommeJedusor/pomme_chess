use crate::binary_mask::{MagicEntry, get_rook_moves_masks};

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
    white: u64,
    black: u64,

    white_pieces: ColorPieces,
    black_pieces: ColorPieces,

    is_white_to_play: bool,
    white_king_side_castle: bool,
    white_queen_side_castle: bool,
    black_king_side_castle: bool,
    black_queen_side_castle: bool,
}

fn get_starting_chessboard() -> ChessBoard {
    return ChessBoard {
        white: 34661728256,
        black: 72567767433218,
        white_pieces: ColorPieces {
            king: 0,
            queens: 0,
            rooks: 33554432,
            bishops: 268435456,
            knights: 34359738368,
            pawns: 0,
        },
        black_pieces: ColorPieces {
            king: 0,
            queens: 0,
            rooks: 72567767433216,
            bishops: 2,
            knights: 0,
            pawns: 0,
        },
        is_white_to_play: true,
        white_king_side_castle: true,
        white_queen_side_castle: true,
        black_king_side_castle: true,
        black_queen_side_castle: true,
    };
    // return ChessBoard {
    //     white: 65535,
    //     black: 18446462598732840960,
    //     white_pieces: ColorPieces {
    //         king: 1152921504606846976,
    //         queens: 576460752303423488,
    //         rooks: 9295429630892703744,
    //         bishops: 2594073385365405696,
    //         knights: 4755801206503243776,
    //         pawns: 71776119061217280,
    //     },
    //     black_pieces: ColorPieces {
    //         king: 16,
    //         queens: 8,
    //         rooks: 129,
    //         bishops: 36,
    //         knights: 66,
    //         pawns: 65280,
    //     },
    //     is_white_to_play: true,
    //     white_king_side_castle: true,
    //     white_queen_side_castle: true,
    //     black_king_side_castle: true,
    //     black_queen_side_castle: true,
    // };
}

impl ChessBoard {
    fn get_fen(&self) -> String {
        // board
        let mut fen_board = String::new();
        for i in 0..64 {
            if self.white_pieces.king & (1 << i) != 0 {
                fen_board.push_str("K");
            } else if self.white_pieces.queens & (1 << i) != 0 {
                fen_board.push_str("Q");
            } else if self.white_pieces.rooks & (1 << i) != 0 {
                fen_board.push_str("R");
            } else if self.white_pieces.bishops & (1 << i) != 0 {
                fen_board.push_str("B");
            } else if self.white_pieces.knights & (1 << i) != 0 {
                fen_board.push_str("N");
            } else if self.white_pieces.pawns & (1 << i) != 0 {
                fen_board.push_str("P");
            } else if self.black_pieces.king & (1 << i) != 0 {
                fen_board.push_str("k");
            } else if self.black_pieces.queens & (1 << i) != 0 {
                fen_board.push_str("q");
            } else if self.black_pieces.rooks & (1 << i) != 0 {
                fen_board.push_str("r");
            } else if self.black_pieces.bishops & (1 << i) != 0 {
                fen_board.push_str("b");
            } else if self.black_pieces.knights & (1 << i) != 0 {
                fen_board.push_str("n");
            } else if self.black_pieces.pawns & (1 << i) != 0 {
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

    fn get_rook_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> u64 {
        let square_mask = &ma.rook_moves_masks_magical_numbers[index as usize].mask;
        let magical_number = &ma.rook_moves_masks_magical_numbers[index as usize].magic_number;
        let board = self.white | self.black;
        let hashkey = (board & square_mask).wrapping_mul(*magical_number) >> 48;
        let rook_moves = ma.rook_mask_blockers_hashmaps[index as usize][hashkey as usize].unwrap();
        //println!("rook");
        //binary_mask::print_mask(rook_moves & self.white ^ rook_moves);
        rook_moves & self.white ^ rook_moves
    }

    fn get_bishop_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> u64 {
        let square_mask = &ma.bishop_moves_masks_magical_numbers[index as usize].mask;
        let magical_number = &ma.bishop_moves_masks_magical_numbers[index as usize].magic_number;
        let board = self.white | self.black;
        let hashkey = (board & square_mask).wrapping_mul(*magical_number) >> 48;
        let bishop_moves =
            ma.bishop_mask_blockers_hashmaps[index as usize][hashkey as usize].unwrap();
        //println!("bishop");
        //binary_mask::print_mask(bishop_moves & self.white ^ bishop_moves);
        bishop_moves & self.white ^ bishop_moves
    }

    fn get_knight_moves(&self, index: u8, ma: &binary_mask::MainHashtables) -> u64 {
        let knight_moves = ma.knight_move_masks[index as usize];
        //println!("knight");
        //binary_mask::print_mask(knight_moves & self.white ^ knight_moves);
        knight_moves & self.white ^ knight_moves
    }

    fn get_moves(&self, ma: binary_mask::MainHashtables) {
        let pieces = if self.is_white_to_play {
            &self.white_pieces
        } else {
            &self.black_pieces
        };

        for i in 0..64 {
            // TODO continue; if square unoccupied by current player
            let index = 1 << i;
            if index & pieces.pawns != 0 {
            } else if index & pieces.king != 0 {
            } else if index & pieces.rooks != 0 {
                self.get_rook_moves(i, &ma);
            } else if index & pieces.bishops != 0 {
                self.get_bishop_moves(i, &ma);
            } else if index & pieces.knights != 0 {
                self.get_knight_moves(i, &ma);
            } else if index & pieces.queens != 0 {
            }
        }
    }
}

fn main() {
    let ma = binary_mask::generate_main_hashtables();
    let chessboard = get_starting_chessboard();
    chessboard.get_moves(ma);
}
