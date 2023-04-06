use serde::Deserialize;

/// Used for regular living entities such as monsters, humans, good or bad, anything living that is
/// not static
#[derive(Deserialize, Debug)]
pub struct Living {
    pub name: String,
    pub sprite: Option<RawSprite>,
    pub view_range: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct RawSprite {
    pub glyph: char,
    pub fg: String,
    pub bg: String,
}
