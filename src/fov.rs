use bracket_pathfinding::prelude::*;

use crate::{Map, Position, State, MAP_HEIGHT, MAP_WIDTH};

impl BaseMap for Map {
    // If the block is transparent you can see through it non opaque
    fn is_opaque(&self, idx: usize) -> bool {
        !self.tiles[idx].is_transparent
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }
}

pub struct ViewShed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool, // whether or not to regenerate the viewshed
}

pub fn update_vision(state: &State) {
    for (_, (viewshed, pos)) in state.world.query::<(&mut ViewShed, &Position)>().iter() {
        if !viewshed.dirty {
            return;
        }
        viewshed.dirty = false;
        viewshed.visible_tiles =
            field_of_view(Point::new(pos.x, pos.y), viewshed.range, &state.map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < MAP_WIDTH as i32 && p.y >= 0 && p.y < MAP_HEIGHT as i32);
    }
}
