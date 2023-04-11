use crate::{actor::Position, Map, tiles::up_stairs};

pub fn find_up_stairs(map: &Map, _connecting_depth: usize) -> Position {
    match map.tiles.iter().position(|tile| tile.sprite.eq(up_stairs().sprite)) {
        None => Position::new(0, 0),
        Some(upstairs_idx) => map.idx_to_pos(upstairs_idx)
    }
}
