use bracket_terminal::prelude::*;
use hecs::*;

mod map;
use map::{generate_overworld_map, xy_to_idx, Map, Tile, MAP_WIDTH};

pub const HEIGHT: usize = 80;
pub const WIDTH: usize = 120;

struct State {
    world: World, // Holds all of our entities
    map: Map,     // Holds the tiles to the world
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        try_move_player(ctx, &self);

        render_map(ctx, &self.map.overworld);
        render_entities(ctx, &self.world);
    }
}

fn try_move_player(ctx: &mut BTerm, state: &State) {
    for (_, pos) in state.world.query::<With<&mut Position, &Player>>().iter() {
        let mut dest_tile = pos.clone();
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::W => {
                    dest_tile.y -= 1;
                }
                VirtualKeyCode::S => {
                    dest_tile.y += 1;
                }
                VirtualKeyCode::A => {
                    dest_tile.x -= 1;
                }
                VirtualKeyCode::D => {
                    dest_tile.x += 1;
                }
                _ => {}
            }
        }

        if let Some(tile) = state.map.overworld.get(xy_to_idx(dest_tile.x, dest_tile.y)) {
            if !tile.is_blocking {
                *pos = dest_tile;
            }
        }
    }
}

/// Renders all entities that have a Position and Sprite component
fn render_entities(ctx: &mut BTerm, world: &World) {
    for (_, (pos, sprite)) in world.query::<(&Position, &CharSprite)>().iter() {
        ctx.set(pos.x, pos.y, sprite.fg, sprite.bg, sprite.glyph);
    }
}
fn render_map(ctx: &mut BTerm, map: &[Tile]) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        ctx.set(x, y, tile.sprite.fg, tile.sprite.bg, tile.sprite.glyph);
        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

type Color = (u8, u8, u8);

#[derive(Clone, Copy)]
pub struct CharSprite {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

impl CharSprite {
    // Create a new sprite, bg defaults to black
    fn new(glyph: char, fg: Color, bg: Option<Color>) -> Self {
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

struct Player;

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/Yayo.png");

fn main() -> BError {
    //Setup terminal renderer
    bracket_terminal::link_resource!(TILE_FONT, "resources/Yayo.png");
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .with_font("Yayo.png", 8, 8)
        .build()?;

    let mut world = World::new();

    world.spawn((
        Position { x: 5, y: 5 },
        CharSprite::new('☺', CYAN, None),
        Player,
    ));

    world.spawn((
        Position::new(10, 12),
        CharSprite {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::new(),
        },
    ));

    let map = generate_overworld_map();
    let gs: State = State { world, map };

    main_loop(context, gs)
}
