/*  Actors are defined as entities who performs actions.
   This file defines the components and systems commonly used by them.
*/
use bracket_terminal::prelude::*;

use crate::{
    fov::ViewShed,
    map::{Destructible, Map},
    tiles::floor_grass,
    BTerm, State,
};

/// Attempts to move an entity's position given it is allowed to move there
/// Returns true if successful in moving
pub fn try_move(map: &mut Map, dest_tile: Position, pos: &mut Position, view: &mut ViewShed) -> bool {
    let idx = dest_tile.0.to_index(map.width);
    if !map.within_bounds(dest_tile.0) {
        return false;
    }
    if let Some(mut tile) = map.tiles.get_mut(idx) {
        if !tile.is_blocking {
            *pos = dest_tile;
            view.dirty = true;
            return true;
        } else {
            match tile.destructible {
                Destructible::ByHand {
                    mut health,
                    dropped_item,
                } => {
                    health -= 1;
                    tile.destructible = Destructible::ByHand { health, dropped_item };
                    if health == 0 {
                        map.tiles[idx] = floor_grass();
                    }
                }
                Destructible::Unbreakable => {}
            };
        }
    }
    false
}

/// Renders all entities that have a Position and Sprite component
pub fn render_entities(ctx: &mut BTerm, state: &State) {
    for (_, (pos, sprite)) in state.world.query::<(&Position, &CharSprite)>().iter() {
        ctx.set(pos.x(), pos.y(), sprite.fg, sprite.bg, sprite.glyph);
    }
}

/// Tag Component that marks the player entity
pub struct Player;

#[derive(Clone, Debug)]
pub struct Position(pub Point);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self(Point::new(x, y))
    }

    // Personal perference to use methods rather than tuple index :P
    pub fn x(&self) -> i32 {
        self.0.x
    }

    pub fn y(&self) -> i32 {
        self.0.y
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
    /// Create a new sprite, bg defaults to black which is useful for items
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

    pub fn eq(&self, other: Self) -> bool {
        self.glyph == other.glyph && self.fg == other.fg && self.bg == other.bg
    }
}
