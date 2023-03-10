use bracket_terminal::prelude::*;

use crate::{
    map::{xy_to_idx, MAP_HEIGHT, MAP_WIDTH},
    BTerm, State, ViewShed, With,
};

pub fn try_move_player(ctx: &mut BTerm, state: &State) {
    for (_, (pos, view)) in state
        .world
        .query::<(With<&mut Position, &Player>, &mut ViewShed)>()
        .iter()
    {
        let mut dest_tile = pos.clone();
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::W => {
                    dest_tile.y = dest_tile.y.saturating_sub(1);
                }
                VirtualKeyCode::S => {
                    dest_tile.y += 1;
                }
                VirtualKeyCode::A => {
                    dest_tile.x = dest_tile.x.saturating_sub(1);
                }
                VirtualKeyCode::D => {
                    dest_tile.x += 1;
                }
                _ => {}
            }
        }

        if let Some(tile) = state.map.tiles.get(xy_to_idx(dest_tile.x, dest_tile.y)) {
            if !tile.is_blocking && within_bounds(dest_tile) {
                *pos = dest_tile;
                view.dirty = true;
            }
        }
    }
}

fn within_bounds(tile_pos: Position) -> bool {
    tile_pos.x < MAP_WIDTH && tile_pos.y < MAP_HEIGHT
}

// Renders all entities that have a Position and Sprite component
pub fn render_entities(ctx: &mut BTerm, state: &State) {
    for (_, (pos, sprite)) in state.world.query::<(&Position, &CharSprite)>().iter() {
        ctx.set(pos.x, pos.y, sprite.fg, sprite.bg, sprite.glyph);
    }
}

// Tag Component that marks the player entity
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct CharSprite {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

type Color = (u8, u8, u8);

impl CharSprite {
    // Create a new sprite, bg defaults to black which is useful for items
    pub fn new(glyph: char, fg: Color, bg: Option<Color>) -> Self {
        match bg {
            Some(bg) => Self {
                glyph: to_cp437(glyph),
                fg: RGB::named(fg),
                bg: RGB::named(bg),
            },
            None => Self {
                glyph: to_cp437(glyph),
                fg: RGB::named(fg),
                bg: RGB::new(),
            },
        }
    }
}
