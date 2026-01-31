use crate::binary_mask::print_mask;

pub mod binary_mask;
pub mod get_moves;
pub mod make_move;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pieces: [u64; 12],

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
        pieces: [
            1 << player_king_indexes[0],
            player_queen_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            player_rook_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            player_bishop_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            player_knight_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            player_pawn_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            1 << opponent_king_indexes[0],
            opponent_queen_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            opponent_rook_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            opponent_bishop_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            opponent_knight_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
            opponent_pawn_indexes
                .iter()
                .map(|x| 1 << x)
                .fold(0, |a, b| a | b),
        ],
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

fn main() {
    let ma = binary_mask::generate_main_hashtables();
    let mut chessboard = get_starting_chessboard();
    println!("{:?}", chessboard.get_fen());
    print_mask(chessboard.board);
    for _ in 0..6 {
        let moves = chessboard.get_moves(&ma);
        println!(
            "{:?}",
            *moves
                .iter()
                .filter(|x| **x != 1 && **x != 405 && **x != 407)
                .next()
                .unwrap()
        );
        chessboard.make_move(
            *moves
                .iter()
                .filter(|x| **x != 1 && **x != 405 && **x != 407)
                .next()
                .unwrap(),
        );
        println!("{:?}", chessboard.get_fen());
    }
}
