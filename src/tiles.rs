use crate::{map::WorldTile, CharSprite};
use bracket_terminal::prelude::{to_cp437, BLACK, BROWN1, BROWN4, RGB, WHITESMOKE};

pub fn wall() -> WorldTile {
    WorldTile {
        is_transparent: false,
        is_blocking: true,
        sprite: CharSprite {
            glyph: to_cp437('#'),
            fg: RGB::named(BROWN1),
            bg: RGB::named(BROWN4),
        },
    }
}

pub fn floor() -> WorldTile {
    WorldTile {
        is_transparent: true,
        is_blocking: false,
        sprite: CharSprite {
            glyph: to_cp437('.'),
            fg: RGB::named(WHITESMOKE),
            bg: RGB::named(BLACK),
        },
    }
}
