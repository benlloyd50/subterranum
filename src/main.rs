use bracket_terminal::prelude::*;
use hecs::*;

mod map;
use map::{generate_overworld_map, Map, MAP_WIDTH, MAP_HEIGHT, render_map};
mod fov;
use fov::{ViewShed, update_vision};
mod tiles;
mod actor;
use actor::{try_move_player, render_entities, Position, CharSprite, Player};

pub const HEIGHT: usize = 80;
pub const WIDTH: usize = 120;

pub struct State {
    world: World, // Holds all of our entities
    map: Map,     // Holds the tiles to the world
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        update_vision(&self);
        try_move_player(ctx, &self);

        render_map(ctx, &self.map.tiles, &self.world);
        render_entities(ctx, &self.world);
    }
}

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
        ViewShed {
            range: 8,
            visible_tiles: Vec::new(),
            dirty: true,
        },
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
