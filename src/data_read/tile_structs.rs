use serde::Deserialize;

use super::living_structs::RawSprite;

#[derive(Deserialize, Debug, Default)]
pub struct TileData {
    pub all: Vec<Tile>
}

#[derive(Deserialize, Debug)]
pub struct Tile {
    pub name: String,
    pub is_transparent: Option<bool>,
    pub is_blocking: Option<bool>,
    pub sprite: Option<RawSprite>,
    pub destructible_info: Option<DestructibleInfo>,
    pub tile_type: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct DestructibleInfo {
    pub by_what: String,
    pub hits: usize,
}
