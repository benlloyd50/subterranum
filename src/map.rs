/* Map.rs is the map generation code and data structures to hold information about the map
 */
use crate::{actor::Position, worldgen::WorldRoom, CharSprite, Config};
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, DistanceAlg, SmallVec};
use bracket_terminal::prelude::{BTerm, Point, PURPLE, WHITESMOKE};
use hecs::Entity;
use serde::{Serialize, Deserialize};

pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 70;

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<WorldTile>,
    pub rooms: Vec<WorldRoom>,
    pub visible: Vec<bool>,
    pub discovered: Vec<bool>,
    pub width: usize,
    pub height: usize,
    pub depth: usize,

    #[serde(skip)]
    pub beings: Vec<Option<Entity>>,
    // #[serde(skip)]
    // pub tile_entity: Vec<Option<Entity>>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WorldTile {
    pub sprite: CharSprite,
    pub is_blocking: bool,
    pub is_transparent: bool,
    // pub destructible: Destructible,
    pub tile_type: TileType,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TileType {
    Floor,
    Wall,
    DownStairs,
    UpStairs,
    Water,
    Special,
    Unknown,
}

// #[derive(Copy, Clone)]
// pub enum Destructible {
//     Unbreakable,
//     ByHand { health: usize, dropped_item: Item },
//     _ByPick { health: usize, dropped_item: Item },
// }

impl Map {
    /// Generates an empty map object, useful for setting up the game before it's started
    pub fn empty() -> Self {
        Self {
            tiles: Vec::new(),
            rooms: Vec::new(),
            visible: Vec::new(),
            discovered: Vec::new(),
            beings: Vec::new(),
            width: 100,
            height: 70,
            depth: 0,
        }
    }

    /// Converts 2d coords to 1d index
    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    /// Converts 1d index to 2d Point
    pub fn idx_to_point(&self, idx: usize) -> Point {
        Point {
            x: (idx % self.width) as i32,
            y: (idx / self.width) as i32,
        }
    }

    /// Converts a 1d index to a 2d Position
    pub fn idx_to_pos(&self, idx: usize) -> Position {
        Position(Point::new(idx % self.width, idx / self.width))
    }

    /// Tiles not blocking and within map boundaries are considered valid
    pub fn valid_exit(&self, start_pos: Point, delta: Point) -> Option<usize> {
        let dest_tile = Point::new(start_pos.x + delta.x, start_pos.y + delta.y);
        if self.within_bounds(dest_tile) {
            let idx = dest_tile.to_index(self.width);
            if !self.tiles[idx].is_blocking {
                return Some(idx);
            }
        }
        None
    }

    /// Returns true if a point is within the bounds of a the map
    pub fn within_bounds(&self, tile_pos: Point) -> bool {
        tile_pos.x < self.width as i32 && tile_pos.y < self.height as i32 && tile_pos.x >= 0 && tile_pos.y >= 0
    }
}

impl WorldTile {
    pub fn empty() -> Self {
        Self {
            is_blocking: false,
            is_transparent: false,
            // destructible: Destructible::Unbreakable,
            sprite: CharSprite::new('?', PURPLE, WHITESMOKE),
            tile_type: TileType::Unknown,
        }
    }
}

impl BaseMap for Map {
    // If the block is transparent you can see through it non opaque
    fn is_opaque(&self, idx: usize) -> bool {
        !self.tiles[idx].is_transparent
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

/// Renders the world from the player perspective
pub fn render_map(ctx: &mut BTerm, map: &Map, config: &Config) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.tiles.iter() {
        if config.dev_mode {
            ctx.set(x, y, tile.sprite.fg, tile.sprite.bg, tile.sprite.glyph);
        } else if map.visible[map.xy_to_idx(x, y)] {
            ctx.set(x, y, tile.sprite.fg, tile.sprite.bg, tile.sprite.glyph);
        } else if map.discovered[map.xy_to_idx(x, y)] {
            ctx.set(
                x,
                y,
                tile.sprite.fg.to_greyscale(),
                tile.sprite.bg.to_greyscale(),
                tile.sprite.glyph,
            );
        }

        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}
