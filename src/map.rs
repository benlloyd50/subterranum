/* Map.rs is the map generation code and data structures to hold information about the map
 */
use crate::{actor::Position, item::Item, CharSprite};
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, DistanceAlg, SmallVec};
use bracket_terminal::prelude::{BTerm, Point};

// TODO: turn these into variables on map and impl fn on to map if they need this
pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 70;

pub struct Map {
    pub tiles: Vec<WorldTile>,
    pub visible: Vec<bool>,
    pub discovered: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone)]
pub struct WorldTile {
    pub sprite: CharSprite,
    pub is_blocking: bool,
    pub is_transparent: bool,
    pub destructible: Destructible,
}

#[derive(Copy, Clone)]
pub enum Destructible {
    ByHand { health: usize, dropped_item: Item },
    Unbreakable,
}

impl Map {
    /// Generates an empty map object, useful for setting up the game before it's started
    pub fn empty() -> Self {
        Self {
            tiles: Vec::new(),
            visible: Vec::new(),
            discovered: Vec::new(),
            width: 100,
            height: 70,
        }
    }

    /// Converts 2d coords to 1d index
    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    /// Converts 2d position to a 1d index
    pub fn point_to_idx(&self, point: Point) -> usize {
        point.x as usize + (point.y as usize * self.width)
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
        Position {
            x: (idx % self.width),
            y: (idx / self.width),
        }
    }

    /// Tiles not blocking and within map boundaries are considered valid
    pub fn valid_exit(&self, start_pos: Point, delta: Point) -> Option<usize> {
        let dest_tile = Point::new(start_pos.x + delta.x, start_pos.y + delta.y);
        if self.within_bounds(dest_tile) {
            let idx = self.point_to_idx(dest_tile);
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
pub fn render_map(ctx: &mut BTerm, map: &Map) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.tiles.iter() {
        // if map.visible[xy_to_idx(x, y)] {
        ctx.set(x, y, tile.sprite.fg, tile.sprite.bg, tile.sprite.glyph);
        // } else if map.discovered[xy_to_idx(x, y)]{
        //     ctx.set(x, y, tile.sprite.fg.to_greyscale(), tile.sprite.bg.to_greyscale(), tile.sprite.glyph);
        // }

        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}
