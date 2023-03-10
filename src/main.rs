use bracket_terminal::prelude::*;
use hecs::*;

mod map;
use map::{generate_map, render_map, Map, MAP_HEIGHT, MAP_WIDTH};
mod fov;
use fov::{update_vision, ViewShed};
mod actor;
mod tiles;
use actor::{render_entities, try_move_player, CharSprite, Player, Position};

pub const HEIGHT: usize = 80;
pub const WIDTH: usize = 120;

pub struct State {
    world: World, // Holds all of our entities
    map: Map,     // Holds the tiles to the world
                  // player_e: Entity // The player's entity for convienent lookup
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        update_vision(self);
        try_move_player(ctx, &self);

        render_map(ctx, &self.map);
        render_entities(ctx, &self);
        ctx.print_right(0, 0, format!("{}", ctx.fps));
    }
}

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/Yayo.png");

fn main() -> BError {
    //Setup terminal renderer
    bracket_terminal::link_resource!(TILE_FONT, "resources/Yayo.png");
    let context = BTermBuilder::simple(WIDTH, HEIGHT)?
        .with_title("Hello Minimal Bracket World")
        .with_font("Yayo.png", 8, 8)
        .with_fullscreen(true) // this could be toggled with a config file! in the future...
        .build()?;

    let mut world = World::new();

    let _player_e = world.spawn((
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

    let map = generate_map();
    let gs: State = State { world, map };

    main_loop(context, gs)
}
