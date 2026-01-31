use crate::binary_mask::print_mask;

pub mod binary_mask;
pub mod get_moves;

struct ColorPieces {
    king: u64,
    queens: u64,
    rooks: u64,
    bishops: u64,
    knights: u64,
    pawns: u64,
}

#[derive(Clone, Copy, Debug)]
enum TypePiece {
    WhiteKing = 0,
    WhiteQueen = 1,
    WhiteRook = 2,
    WhiteBishop = 3,
    WhiteKnight = 4,
    WhitePawn = 5,
    BlackKing = 6,
    BlackQueen = 7,
    BlackRook = 8,
    BlackBishop = 9,
    BlackKnight = 10,
    BlackPawn = 11,
    Empty = 12,
}

struct ChessBoard {
    board: u64,
    white: u64,
    black: u64,

    white_pieces: ColorPieces,
    black_pieces: ColorPieces,

    pieces_by_index: [TypePiece; 64],

    is_white_to_play: bool,
    player_king_side_castle: bool,
    player_queen_side_castle: bool,
    opponent_king_side_castle: bool,
    opponent_queen_side_castle: bool,

    en_passant: u64,
}

fn get_starting_chessboard() -> ChessBoard {
    let mut pieces_by_index = [TypePiece::Empty; 64];
    // player
    let player_rook_indexes = [56, 63];
    let player_knight_indexes = [57, 62];
    let player_bishop_indexes = [58, 61];
    let player_queen_indexes = [59];
    let player_king_indexes = [60];
    let player_pawn_indexes = [48, 49, 50, 51, 52, 53, 54, 55];
    let mut player_board = 0;
    for (type_piece, indexes) in [
        (TypePiece::WhiteRook, player_rook_indexes.iter()),
        (TypePiece::WhiteKnight, player_knight_indexes.iter()),
        (TypePiece::WhiteBishop, player_bishop_indexes.iter()),
        (TypePiece::WhiteQueen, player_queen_indexes.iter()),
        (TypePiece::WhiteKing, player_king_indexes.iter()),
        (TypePiece::WhitePawn, player_pawn_indexes.iter()),
    ] {
        for index in indexes {
            player_board |= 1 << index;
            pieces_by_index[*index as usize] = type_piece;
        }
    }
    // opponent
    let opponent_rook_indexes = [0, 7];
    let opponent_knight_indexes = [1, 6];
    let opponent_bishop_indexes = [2, 5];
    let opponent_queen_indexes = [3];
    let opponent_king_indexes = [4];
    let opponent_pawn_indexes = [8, 9, 10, 11, 12, 13, 14, 15];
    let mut opponent_board = 0;
    for (type_piece, indexes) in [
        (TypePiece::BlackRook, opponent_rook_indexes.iter()),
        (TypePiece::BlackKnight, opponent_knight_indexes.iter()),
        (TypePiece::BlackBishop, opponent_bishop_indexes.iter()),
        (TypePiece::BlackQueen, opponent_queen_indexes.iter()),
        (TypePiece::BlackKing, opponent_king_indexes.iter()),
        (TypePiece::BlackPawn, opponent_pawn_indexes.iter()),
    ] {
        for index in indexes {
            opponent_board |= 1 << index;
            pieces_by_index[*index as usize] = type_piece;
        }
    }
    return ChessBoard {
        board: player_board | opponent_board,
        white: player_board,
        black: opponent_board,
        white_pieces: ColorPieces {
            king: 1 << player_king_indexes[0],
            queens: player_queen_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            rooks: player_rook_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            bishops: player_bishop_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            knights: player_knight_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            pawns: player_pawn_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
        },
        black_pieces: ColorPieces {
            king: 1 << opponent_king_indexes[0],
            queens: opponent_queen_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            rooks: opponent_rook_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            bishops: opponent_bishop_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            knights: opponent_knight_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            pawns: opponent_pawn_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
        },
        pieces_by_index: pieces_by_index,
        is_white_to_play: true,
        player_king_side_castle: true,
        player_queen_side_castle: true,
        opponent_king_side_castle: true,
        opponent_queen_side_castle: true,
        en_passant: 0,
    };
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

fn main() {
    let ma = binary_mask::generate_main_hashtables();
    let chessboard = get_starting_chessboard();
    println!("{:?}", chessboard.get_fen());
    print_mask(chessboard.board);
    println!("{:?}", chessboard.pieces_by_index);
    let moves = chessboard.get_moves(ma);
    println!("{:?}", moves);
}
