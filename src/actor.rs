/*  Actors are defined as entities who performs actions.
   This file defines the components and systems commonly used by them.
*/
use bracket_terminal::prelude::*;
use hecs::Entity;
use serde::Deserialize;

use crate::{
    data_read::named_tile,
    fov::ViewShed,
    map::{Destructible, Map, TileType},
    BTerm, State,
};

pub fn try_descend(map: &Map, player_pos: &Position) -> bool {
    map.tiles[player_pos.0.to_index(map.width)].tile_type == TileType::DownStairs
}

pub fn try_ascend(map: &Map, player_pos: &Position, depth: usize, delta: usize) -> bool {
    if depth == 0 && delta == 1 {
        false
    } else if map.tiles[player_pos.0.to_index(map.width)].tile_type == TileType::UpStairs {
        true
    } else {
        false
    }
}

pub enum MoveResult {
    Acted(String),
    Attack(Entity),
    // Mine(),
    InvalidMove(String),
}

/// Attempts to move an entity's position given it is allowed to move there
/// Returns true if successful in moving
pub fn try_move(
    map: &mut Map,
    dest_tile: &Position,
    pos: &mut Position,
    view: &mut ViewShed,
    who: Entity, // the entity that is moving
) -> MoveResult {
    if !map.within_bounds(dest_tile.0) {
        return MoveResult::InvalidMove(format!("{},{} is out of bounds", dest_tile.0.x, dest_tile.0.y));
    }

    let dest_idx = dest_tile.0.to_index(map.width);
    if let Some(target) = map.beings[dest_idx] {
        return MoveResult::Attack(target);
    }

    if let Some(mut tile) = map.tiles.get_mut(dest_idx) {
        view.dirty = true; // make it dirty so the vision is updated definitely
        if !tile.is_blocking {
            let idx = pos.0.to_index(map.width);
            *pos = dest_tile.clone();
            map.beings[idx] = None;
            map.beings[dest_idx] = Some(who);
            return MoveResult::Acted("".to_string());
        } else {
            match tile.destructible {
                Destructible::ByHand {
                    mut health,
                    dropped_item,
                } => {
                    health -= 1;
                    tile.destructible = Destructible::ByHand { health, dropped_item };
                    if health == 0 {
                        map.tiles[dest_idx] = named_tile("Grass Floor");
                    }
                    return MoveResult::Acted("".to_string());
                }
                Destructible::Unbreakable => ("That tile is unbreakable.", false),
                Destructible::_ByPick { .. } => {
                    unimplemented!("Pickaxe not ready for use")
                }
            };
        }
    }
    MoveResult::InvalidMove("No tile".to_string())
}

/// Renders all entities that have a Position and Sprite component
pub fn render_entities(ctx: &mut BTerm, state: &State) {
    if let Some((_, (view, _))) = state.world.query::<(&ViewShed, &Player)>().iter().next() {
        for (_, (pos, sprite)) in state.world.query::<(&Position, &CharSprite)>().iter() {
            if view.visible_tiles.contains(&pos.0) || state.config.dev_mode {
                ctx.set(pos.x(), pos.y(), sprite.fg, sprite.bg, sprite.glyph);
            }
        }
    }
}

/// Tag Component that marks the player entity
#[derive(Deserialize, Debug)]
pub struct Player;

#[derive(Deserialize, Debug)]
pub struct Name(pub String);

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

#[derive(Clone, Copy, Debug)]
pub struct CharSprite {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

type Color = (u8, u8, u8);

impl CharSprite {
    /// Create a new sprite, bg defaults to black which is useful for items
    pub fn with_color(glyph: char, fg: Color, bg: Option<Color>) -> Self {
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

    pub fn new(glyph: char, fg: Color, bg: Color) -> Self {
        Self {
            glyph: to_cp437(glyph),
            fg: RGB::named(fg),
            bg: RGB::named(bg),
        }
    }

    pub fn rgb(glyph: char, fg: RGB, bg: RGB) -> Self {
        Self {
            glyph: to_cp437(glyph),
            fg,
            bg,
        }
    }

    pub fn eq(&self, other: Self) -> bool {
        self.glyph == other.glyph && self.fg == other.fg && self.bg == other.bg
    }
}
