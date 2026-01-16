use std::usize;

pub struct MainHashtables {
    pub rook_mask_blockers_hashmaps: Vec<Vec<Option<u64>>>,
    pub rook_moves_masks_magical_numbers: [MagicEntry; 64],
    pub bishop_mask_blockers_hashmaps: Vec<Vec<Option<u64>>>,
    pub bishop_moves_masks_magical_numbers: [MagicEntry; 64],
    pub knight_move_masks: [u64; 64],
}

#[derive(Debug)]
pub struct MagicEntry {
    pub mask: u64,
    pub magic_number: u64,
}

pub fn get_rook_moves_masks() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for i in 0..64 {
        let mut rook_moves = 0;
        let x = i % 8;
        let y = i / 8;

        for j in 1..7 {
            if j != x {
                rook_moves |= 1 << (j + y * 8);
            }
            if j != y {
                rook_moves |= 1 << (x + j * 8);
            }
        }
        moves[i] = rook_moves;
    }
    moves
}

pub fn get_rook_moves_masks_collision(index: usize, mask: &u64) -> u64 {
    let x = index as u16 % 8;
    let y = index as u16 / 8;
    let mut collision_mask = 0;
    let left = (0..x)
        .filter(|x| 1 << (y * 8 + x) & mask != 0)
        .max()
        .unwrap_or(0);
    let right = ((x + 1)..8)
        .filter(|x| 1 << (y * 8 + x) & mask != 0)
        .min()
        .unwrap_or(7);
    let top = (0..y)
        .filter(|y| 1 << (y * 8 + x) & mask != 0)
        .max()
        .unwrap_or(0);
    let bottom = ((y + 1)..8)
        .filter(|y| 1 << (y * 8 + x) & mask != 0)
        .min()
        .unwrap_or(7);
    for i in left..x {
        collision_mask |= 1 << (y * 8 + i);
    }
    for i in (x + 1)..=right {
        collision_mask |= 1 << (y * 8 + i);
    }
    for i in top..y {
        collision_mask |= 1 << (i * 8 + x);
    }
    for i in (y + 1)..=bottom {
        collision_mask |= 1 << (i * 8 + x);
    }
    collision_mask
}

pub fn get_rook_moves_masks_magical_numbers(
    mask_blockers_hashmaps: &mut Vec<Vec<Option<u64>>>,
) -> [MagicEntry; 64] {
    assert!(mask_blockers_hashmaps.len() == 64 && mask_blockers_hashmaps[0].len() == 65536);
    let mut magical_numbers: [Option<MagicEntry>; 64] = [const { None }; 64];
    let moves_masks = get_rook_moves_masks();
    for (i, moves_mask) in moves_masks.iter().enumerate() {
        // get mask indexes
        let mut mask_indexes: [u8; 12] = [0; 12];
        for (j, index) in (0..64).filter(|x| 1 << x & moves_mask != 0).enumerate() {
            assert!(j < 12);
            mask_indexes[j] = index;
        }

        // get mask blockers
        let mut mask_blockers: [u64; 4096] = [0; 4096];
        for j in 0..4096 {
            let mut mask_blocker = 0;
            for (k, index) in mask_indexes.iter().enumerate() {
                if 1 << k & j != 0 {
                    mask_blocker |= 1 << index;
                }
            }
            mask_blockers[j] = mask_blocker;
        }

        // find magic number
        while true {
            let j = rand::random_range(0..=18446744073709551615)
                & rand::random_range(0..=18446744073709551615)
                & rand::random_range(0..=18446744073709551615);
            let mut is_valid = true;
            for mask_blocker in mask_blockers.iter() {
                let hashkey = mask_blocker.wrapping_mul(j) >> 48;
                let colision = get_rook_moves_masks_collision(i, mask_blocker);
                if mask_blockers_hashmaps[i][hashkey as usize].is_some_and(|x| x != colision) {
                    is_valid = false;
                    break;
                }
                mask_blockers_hashmaps[i][hashkey as usize] = Some(colision);
            }
            if is_valid {
                magical_numbers[i] = Some(MagicEntry {
                    mask: *moves_mask,
                    magic_number: j,
                });
                break;
            }
            // reset mask_blockers_hashmaps
            for j in 0..65536 {
                mask_blockers_hashmaps[i][j] = None;
            }
        }
    }
    magical_numbers.map(|x| x.unwrap())
}

pub fn get_bishop_moves_masks() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for i in 0..64 {
        let mut bishop_moves = 0;
        let x = i % 8;
        let y = i / 8;

        for j in 1..7 {
            if j < x && j < y {
                bishop_moves |= 1 << ((y - j) * 8 + x - j);
            }
            if j + x < 7 && j + y < 7 {
                bishop_moves |= 1 << ((y + j) * 8 + x + j);
            }
            if j < x && j + y < 7 {
                bishop_moves |= 1 << ((y + j) * 8 + x - j);
            }
            if j + x < 7 && j < y {
                bishop_moves |= 1 << ((y - j) * 8 + x + j);
            }
        }
        moves[i] = bishop_moves;
    }
    moves
}

pub fn get_bishop_moves_masks_collision(index: usize, mask: &u64) -> u64 {
    let x = index as u16 % 8;
    let y = index as u16 / 8;
    let mut collision_mask = 0;
    // top left
    let mut i = 1;
    while x >= i && y >= i {
        let square_index = (y - i) * 8 + x - i;
        if 1 << square_index & mask != 0 {
            break;
        }
        collision_mask |= 1 << square_index;
        i += 1;
    }
    // top right
    let mut i = 1;
    while x + i <= 7 && y >= i {
        let square_index = (y - i) * 8 + x + i;
        if 1 << square_index & mask != 0 {
            break;
        }
        collision_mask |= 1 << square_index;
        i += 1;
    }
    // bottom left
    let mut i = 1;
    while x >= i && y + i <= 7 {
        let square_index = (y + i) * 8 + x - i;
        if 1 << square_index & mask != 0 {
            break;
        }
        collision_mask |= 1 << square_index;
        i += 1;
    }
    // top right
    let mut i = 1;
    while x + i <= 7 && y + i <= 7 {
        let square_index = (y + i) * 8 + x + i;
        if 1 << square_index & mask != 0 {
            break;
        }
        collision_mask |= 1 << square_index;
        i += 1;
    }
    collision_mask
}

pub fn get_bishop_moves_masks_magical_numbers(
    mask_blockers_hashmaps: &mut Vec<Vec<Option<u64>>>,
) -> [MagicEntry; 64] {
    assert!(mask_blockers_hashmaps.len() == 64 && mask_blockers_hashmaps[0].len() == 65536);
    let mut magical_numbers: [Option<MagicEntry>; 64] = [const { None }; 64];
    let moves_masks = get_bishop_moves_masks();
    for (i, moves_mask) in moves_masks.iter().enumerate() {
        // get mask indexes
        let mut mask_indexes: [u8; 13] = [0; 13];
        for (j, index) in (0..64).filter(|x| 1 << x & moves_mask != 0).enumerate() {
            assert!(j < 13);
            mask_indexes[j] = index;
        }

        // get mask blockers
        let mut mask_blockers: [u64; 4096] = [0; 4096];
        for j in 0..4096 {
            let mut mask_blocker = 0;
            for (k, index) in mask_indexes.iter().enumerate() {
                if 1 << k & j != 0 {
                    mask_blocker |= 1 << index;
                }
            }
            mask_blockers[j] = mask_blocker;
        }

        // find magic number
        while true {
            let j = rand::random_range(0..=18446744073709551615)
                & rand::random_range(0..=18446744073709551615)
                & rand::random_range(0..=18446744073709551615);
            let mut is_valid = true;
            for mask_blocker in mask_blockers.iter() {
                let hashkey = mask_blocker.wrapping_mul(j) >> 48;
                let colision = get_bishop_moves_masks_collision(i, mask_blocker);
                if mask_blockers_hashmaps[i][hashkey as usize].is_some_and(|x| x != colision) {
                    is_valid = false;
                    break;
                }
                mask_blockers_hashmaps[i][hashkey as usize] = Some(colision);
            }
            if is_valid {
                magical_numbers[i] = Some(MagicEntry {
                    mask: *moves_mask,
                    magic_number: j,
                });
                break;
            }
            // reset mask_blockers_hashmaps
            for j in 0..65536 {
                mask_blockers_hashmaps[i][j] = None;
            }
        }
    }
    magical_numbers.map(|x| x.unwrap())
}

pub fn get_knight_moves_masks() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for i in 0..64 {
        let mut knight_moves = 0;
        let x: i32 = i as i32 % 8;
        let y: i32 = i as i32 / 8;

        for (xa, ya) in [
            (1, 2),
            (2, 1),
            (-1, -2),
            (-2, -1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, -1),
        ] {
            if 0 <= x + xa && x + xa < 8 && 0 <= y + ya && y + ya < 8 {
                knight_moves |= 1 << (y + ya) * 8 + x + xa;
            }
        }
        moves[i] = knight_moves;
    }
    moves
}

pub fn generate_main_hashtables() -> MainHashtables {
    let mut rook_mask_blockers_hashmaps: Vec<Vec<Option<u64>>> = vec![vec![None; 65536]; 64];
    let rook_moves_masks_magical_numbers =
        get_rook_moves_masks_magical_numbers(&mut rook_mask_blockers_hashmaps);
    let mut bishop_mask_blockers_hashmaps: Vec<Vec<Option<u64>>> = vec![vec![None; 65536]; 64];
    let bishop_moves_masks_magical_numbers =
        get_bishop_moves_masks_magical_numbers(&mut bishop_mask_blockers_hashmaps);
    MainHashtables {
        rook_mask_blockers_hashmaps: rook_mask_blockers_hashmaps,
        rook_moves_masks_magical_numbers: rook_moves_masks_magical_numbers,
        bishop_mask_blockers_hashmaps: bishop_mask_blockers_hashmaps,
        bishop_moves_masks_magical_numbers: bishop_moves_masks_magical_numbers,
        knight_move_masks: get_knight_moves_masks(),
    }
}

pub fn print_mask(mask: u64) {
    let x = format!("{mask:b}");
    let mut x = "0".repeat(64 - x.len()) + &x;
    for i in (1..8).rev() {
        x.insert(i * 8, '\n');
    }
    println!("{}", x);
}
