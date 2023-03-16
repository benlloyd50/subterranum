use bracket_pathfinding::prelude::*;

use crate::{Position, State};

pub struct ViewShed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool, // whether or not to regenerate the viewshed
}

impl ViewShed {
    pub fn new(range: i32) -> Self {
        Self {
            range,
            visible_tiles: Vec::new(),
            dirty: true,
        }
    }
}

pub fn update_vision(state: &mut State) {
    for (_, (viewshed, pos)) in state.world.query::<(&mut ViewShed, &Position)>().iter() {
        if !viewshed.dirty {
            return;
        }
        viewshed.dirty = false;
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &state.map);
        viewshed.visible_tiles.retain(|p| state.map.within_bounds(*p));

        for tile in state.map.visible.iter_mut() {
            *tile = false;
        }
        for point in viewshed.visible_tiles.iter() {
            let idx = state.map.point_to_idx(*point);
            state.map.discovered[idx] = true;
            state.map.visible[idx] = true;
        }
    }
}
