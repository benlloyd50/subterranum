use bracket_terminal::prelude::*;
use hecs::*;

pub const HEIGHT: usize = 80;
pub const WIDTH: usize = 120;

struct State {
    world: World, // Holds all of our entities
    tiles: Vec<Tile>, // Holds the tiles to the world
}

// Converts 2d coords to 1d index
fn xy_to_idx(x: usize, y: usize) -> usize {
    x + (y * WIDTH)
}

// Converts 1d index to 2d coords
fn idx_to_xy(idx: usize) -> (usize, usize) {
    (idx / WIDTH, idx % WIDTH)
}

#[derive(Copy, Clone)]
struct Tile {
    tile_type: TileType,
    is_blocking: bool,
}


#[derive(Copy, Clone)]
enum TileType {
    Floor,
    Wall,
}

fn generate_overworld_map() -> Vec<Tile> {
    let mut map = vec![Tile{tile_type: TileType::Wall, is_blocking: true}; WIDTH * HEIGHT];
    
    for x in 1..20 {
        for y in 1..15 {
            map[xy_to_idx(x, y)] = Tile {tile_type: TileType::Floor, is_blocking: false};
        }
    }


    map
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        try_move_player(ctx, &self.world);
        

        //Draw order will be map then entities
        render_map(ctx, &self.tiles);
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

/// Renders all entities that have a Position and Sprite component
fn render_entities(ctx: &mut BTerm, world: &World) {
    for (_, (pos, sprite)) in world.query::<(&Position, &Sprite)>().iter() {
        ctx.set(pos.x, pos.y, sprite.fg, sprite.bg, sprite.glyph);
    }
}
fn render_map(ctx: &mut BTerm, map: &[Tile]) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        match tile.tile_type {
            TileType::Wall => ctx.set(x, y, RGB::named(ROSYBROWN), RGB::named(BROWN1), to_cp437('#')),
            TileType::Floor => ctx.set(x, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('.')),
        } 

        x += 1;
        if x == WIDTH {
            x = 0;
            y += 1;
        }
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

    let tiles = generate_overworld_map();
    let gs: State = State { world, tiles };

    main_loop(context, gs)
}
