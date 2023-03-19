use crate::{
    item::Item,
    map::{Destructible, WorldTile},
    CharSprite,
};
use bracket_terminal::prelude::*;

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
    }
}
