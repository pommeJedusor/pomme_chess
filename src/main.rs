use crate::{binary_mask::print_mask, notation::get_notation_from_move};

pub mod binary_mask;
pub mod get_moves;
pub mod make_move;
pub mod notation;

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
    players: [u64; 2],

    pieces: [u64; 13],

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
        players: [opponent_board, player_board],
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
            0,
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

fn show_current_chessboard_state(chessboard: &ChessBoard) {
    println!("chessboard");
    print_mask(chessboard.board);
    println!("white pieces");
    print_mask(chessboard.players[1]);
    println!("black pieces");
    print_mask(chessboard.players[0]);
    for (i, name) in [
        "white king",
        "white queen",
        "white rook",
        "white bishop",
        "white knight",
        "white pawn",
        "black king",
        "black queen",
        "black rook",
        "black bishop",
        "black knight",
        "black pawn",
    ]
    .iter()
    .enumerate()
    {
        println!("{}", name);
        print_mask(chessboard.pieces[i]);
    }
}

fn main() {
    let ma = binary_mask::generate_main_hashtables();
    let mut chessboard = get_starting_chessboard();
    println!("{:?}", chessboard.get_fen());
    // white en passant
    //chessboard.make_move(24 | (48 << 6));
    //chessboard.make_move(25 | (9 << 6));
    //chessboard.make_move(chessboard.get_moves(&ma)[1]);
    // black en passant
    //chessboard.is_white_to_play = false;
    //chessboard.make_move(32 | (8 << 6));
    //chessboard.make_move(33 | (49 << 6));
    //chessboard.make_move(chessboard.get_moves(&ma)[22]);
    // white promotion
    //chessboard.make_move(8 | (48 << 6));
    //chessboard.make_move(16 | (0 << 6));
    //let moves = chessboard.get_moves(&ma);
    //println!("{:?}", moves);
    //chessboard.make_move(moves[7]);
    //show_current_chessboard_state(&chessboard);
    // black promotion
    chessboard.is_white_to_play = false;
    chessboard.make_move(48 | (8 << 6));
    chessboard.make_move(40 | (56 << 6));
    let moves = chessboard.get_moves(&ma);
    println!("{:?}", moves);
    chessboard.make_move(moves[23]);
    show_current_chessboard_state(&chessboard);
}
