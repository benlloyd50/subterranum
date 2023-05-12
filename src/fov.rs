use bracket_pathfinding::prelude::*;

use crate::{Player, Position, State};

pub struct ViewShed {
    pub visible_tiles: Vec<Point>,
    pub range: u32,
    pub dirty: bool, // whether or not to regenerate the viewshed
}

impl ViewShed {
    pub fn new(range: u32) -> Self {
        Self {
            range,
            visible_tiles: Vec::new(),
            dirty: true,
        }
    }
}

pub fn update_vision(state: &mut State) {
    for (_, (viewshed, pos, player)) in state
        .world
        .query::<(&mut ViewShed, &Position, Option<&Player>)>()
        .iter()
    {
        if !viewshed.dirty {
            return;
        }
        viewshed.dirty = false;
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(pos.0, viewshed.range as i32, &state.map);
        viewshed.visible_tiles.retain(|p| state.map.within_bounds(*p));

        if let Some(_) = player {
            for tile in state.visible.iter_mut() {
                *tile = false;
            }
            for point in viewshed.visible_tiles.iter() {
                let idx = point.to_index(state.map.width);
                state.map.discovered[idx] = true;
                state.visible[idx] = true;
            }
        }
    }
}
