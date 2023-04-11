use crate::{
    item::Item,
    map::{Destructible, WorldTile, TileType},
    CharSprite,
};
use bracket_terminal::prelude::*;

/* Tiles have lots of different kind of information
 * Type 1 - Almost normal behavior for a tile since it is on all of them
 * - Breakble - By what, how many hits before broken, does it drop anything when broken
 *
 * Type 2 - Exceptional behavior, stored on a few tiles/items, they are entities
 * - Door - state of the door, locked?
 * - Stairs
 * - Examinable - description of the tile,
 */

pub fn wall_stone() -> WorldTile {
    WorldTile {
        is_transparent: false,
        is_blocking: true,
        sprite: CharSprite {
            glyph: to_cp437('#'),
            fg: RGB::named(SANDY_BROWN),
            bg: RGB::named(BROWN4),
        },
        destructible: Destructible::ByHand {
            health: 4,
            dropped_item: Item {},
        },
        tile_type: TileType::Wall,
    }
}

pub fn wall_adamnatite() -> WorldTile {
    WorldTile {
        is_transparent: false,
        is_blocking: true,
        sprite: CharSprite {
            glyph: to_cp437('#'),
            fg: RGB::named(WHITESMOKE),
            bg: RGB::named(PURPLE2),
        },
        destructible: Destructible::Unbreakable,
        tile_type: TileType::Wall,
    }
}

pub fn up_stairs() -> WorldTile {
    WorldTile {
        is_transparent: false,
        is_blocking: false,
        sprite: CharSprite::with_color('<', WHITESMOKE, None),
        destructible: Destructible::Unbreakable,
        tile_type: TileType::UpStairs,
    }
}

pub fn down_stairs() -> WorldTile {
    WorldTile {
        is_transparent: false,
        is_blocking: false,
        sprite: CharSprite::with_color('>', WHITESMOKE, None),
        destructible: Destructible::Unbreakable,
        tile_type: TileType::DownStairs,
    }
}

#[allow(dead_code)]
pub fn deep_water() -> WorldTile {
    WorldTile {
        is_transparent: true,
        is_blocking: true,
        sprite: CharSprite {
            glyph: to_cp437('~'),
            fg: RGB::named(WHITE_SMOKE),
            bg: RGB::named(ROYAL_BLUE),
        },
        destructible: Destructible::Unbreakable,
        tile_type: TileType::Water,
    }
}

pub fn floor_stone() -> WorldTile {
    WorldTile {
        is_transparent: true,
        is_blocking: false,
        sprite: CharSprite {
            glyph: to_cp437('.'),
            fg: RGB::named(WHITESMOKE),
            bg: RGB::named(LIGHTSLATEGRAY),
        },
        destructible: Destructible::Unbreakable,
        tile_type: TileType::Floor,
    }
}

// SLATEBLUE coming soon...

#[allow(dead_code)]
pub fn floor_grass() -> WorldTile {
    WorldTile {
        is_transparent: true,
        is_blocking: false,
        sprite: CharSprite {
            glyph: to_cp437('.'),
            fg: RGB::named(WHITESMOKE),
            bg: RGB::named(FOREST_GREEN),
        },
        destructible: Destructible::Unbreakable,
        tile_type: TileType::Floor,
    }
}

pub fn lush_brush() -> WorldTile {
    WorldTile {
        is_transparent: false,
        is_blocking: false,
        sprite: CharSprite {
            glyph: to_cp437('½'),
            fg: RGB::named(GREEN3),
            bg: RGB::named(DARK_GREEN),
        },
        destructible: Destructible::Unbreakable,
        tile_type: TileType::Special,
    }
}
