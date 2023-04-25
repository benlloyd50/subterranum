use crate::{actor::Position, map::TileType, Map};
use bracket_terminal::FontCharType;
use rand::Rng;

pub fn find_tile_from_type(map: &Map, _connecting_depth: usize, tile_type: &TileType) -> Position {
    match map.tiles.iter().position(|tile| tile.tile_type == *tile_type) {
        None => Position::new(0, 0),
        Some(upstairs_idx) => map.idx_to_pos(upstairs_idx),
    }
}

// const INNER_BORDER_TILE: u16 = 32;
static VARIETY_TILE: [u16; 4] = [96, 32, 35, 39];
pub fn wall_glyph(map: &Map, x: usize, y: usize) -> FontCharType {
    let mut mask: u8 = 0;

    if y != 0 && is_revealed_and_wall(map, x, y - 1) {
            mask += 1;
    }
    if y < map.height - 1 && is_revealed_and_wall(map, x, y + 1) {
            mask += 2;
    }
    if x != 0 && is_revealed_and_wall(map, x - 1, y) {
            mask += 4;
    }
    if x < map.width - 1 && is_revealed_and_wall(map, x + 1, y) {
            mask += 8;
    }

    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 188,  // Wall to the north and west
        6 => 187,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 200,  // Wall to the north and east
        10 => 201, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 202, // Wall to the east, west, and south
        14 => 203, // Wall to the east, west, and north

        _ => {
            let idx = rand::thread_rng().gen::<usize>() % VARIETY_TILE.len();
            match VARIETY_TILE.get(idx) {
                Some(&tile) => tile,
                None => 98,
            }
        } // ╬ Wall on all sides or we missed one
    }
}

fn is_revealed_and_wall(map: &Map, x: usize, y: usize) -> bool {
    let idx = map.xy_to_idx(x, y);
    map.tiles[idx].tile_type == TileType::Wall //&& map.visible[idx]
}
