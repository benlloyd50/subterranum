use crate::CharSprite;
use bracket_terminal::prelude::{to_cp437, BLACK, BROWN1, BROWN4, RGB, WHITESMOKE};

pub fn wall() -> CharSprite {
    CharSprite {
        glyph: to_cp437('#'),
        fg: RGB::named(BROWN1),
        bg: RGB::named(BROWN4),
    }
}

pub fn floor() -> CharSprite {
    CharSprite {
        glyph: to_cp437('.'),
        fg: RGB::named(WHITESMOKE),
        bg: RGB::named(BLACK),
    }
}
