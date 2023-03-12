use crate::{map::WorldTile, CharSprite};
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
    }
}

pub fn deep_water() -> WorldTile {
    WorldTile {
        is_transparent: true,
        is_blocking: true,
        sprite: CharSprite {
            glyph: to_cp437('~'),
            fg: RGB::named(WHITE_SMOKE),
            bg: RGB::named(ROYAL_BLUE),
        },
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
    }
}
