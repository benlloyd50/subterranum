use crate::{actor::Position, map::TileType, Map};

pub fn find_tile_from_type(map: &Map, _connecting_depth: usize, tile_type: &TileType) -> Position {
    match map.tiles.iter().position(|tile| tile.tile_type == *tile_type) {
        None => {
            println!("Found no {:?}", tile_type);
            Position::new(0, 0)
        }
        Some(upstairs_idx) => map.idx_to_pos(upstairs_idx),
    }
}
