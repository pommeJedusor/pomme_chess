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
        white: 5,
        black: 0,
        white_pieces: ColorPieces {
            king: 0,
            queens: 0,
            rooks: 5,
            bishops: 0,
            knights: 0,
            pawns: 0,
        },
        black_pieces: ColorPieces {
            king: 0,
            queens: 0,
            rooks: 0,
            bishops: 0,
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

    fn get_rook_moves(
        &self,
        index: u8,
        rook_moves_masks_magical_numbers: &[MagicEntry; 64],
        mask_blockers_hashmaps: &Vec<Vec<Option<u64>>>,
    ) {
        let square_mask = &rook_moves_masks_magical_numbers[index as usize].mask;
        let magical_number = &rook_moves_masks_magical_numbers[index as usize].magic_number;
        let board = self.white | self.black;
        let hashkey = (board & square_mask).wrapping_mul(*magical_number) >> 48;
        let rook_moves = mask_blockers_hashmaps[index as usize][hashkey as usize].unwrap();
        println!("{}", rook_moves & board ^ rook_moves);
    }

    fn get_moves(
        &self,
        rook_moves_masks_magical_numbers: &[MagicEntry; 64],
        mask_blockers_hashmaps: &Vec<Vec<Option<u64>>>,
    ) {
        let pieces = if self.is_white_to_play {
            &self.white_pieces
        } else {
            &self.black_pieces
        };

        for i in 0..64 {
            let index = 1 << i;
            if index & pieces.pawns != 0 {
            } else if index & pieces.king != 0 {
            } else if index & pieces.rooks != 0 {
                self.get_rook_moves(
                    i,
                    &rook_moves_masks_magical_numbers,
                    &mask_blockers_hashmaps,
                );
            } else if index & pieces.bishops != 0 {
            } else if index & pieces.knights != 0 {
            } else if index & pieces.queens != 0 {
            }
        }
    }
}

fn main() {
    //let chessboard = get_starting_chessboard();
    //let fen = chessboard.get_fen();
    //println!("{:?}", fen);
    let mut mask_blockers_hashmaps: Vec<Vec<Option<u64>>> = vec![vec![None; 65536]; 64];
    println!("generating rook magical numbers");
    let rook_moves_masks_magical_numbers =
        binary_mask::get_rook_moves_masks_magical_numbers(&mut mask_blockers_hashmaps);
    println!("generated rook magical numbers");
    let mut chess_board = get_starting_chessboard();
    chess_board.get_moves(&rook_moves_masks_magical_numbers, &mask_blockers_hashmaps);
}
