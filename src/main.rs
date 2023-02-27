use bracket_terminal::prelude::*;
use hecs::*;

struct State {
    world: World, // Holds all of our entities
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        try_move_player(ctx, &self.world);
        render_entities(ctx, &self.world);
    }
}

fn try_move_player(ctx: &mut BTerm, world: &World) {
    for (_, pos) in world.query::<With<&mut Position, &Player>>().iter() {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::W => {
                    pos.y -= 1;
                }
                VirtualKeyCode::S => {
                    pos.y += 1;
                }
                VirtualKeyCode::A => {
                    pos.x -= 1;
                }
                VirtualKeyCode::D => {
                    pos.x += 1;
                }
                _ => {}
            }
        }
    }
}

fn render_entities(ctx: &mut BTerm, world: &World) {
    for (_, (pos, sprite)) in world.query::<(&Position, &Sprite)>().iter() {
        ctx.set(pos.x, pos.y, sprite.fg, sprite.bg, sprite.glyph);
    }
}

struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

type Color = (u8, u8, u8);

struct Sprite {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

impl Sprite {
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
        Sprite::new('☺', CYAN, None),
        Player,
    ));

    world.spawn((
        Position::new(10, 12),
        Sprite {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::new(),
        },
    ));

    let gs: State = State { world };

    main_loop(context, gs)
}
