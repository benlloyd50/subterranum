use bracket_noise::prelude::*;
/// Map.rs is the map generation code and data structures to hold information about the map
use bracket_terminal::prelude::{to_cp437, BTerm, Point, PURPLE, RGB, YELLOW};

use crate::{tiles::*, CharSprite, Player, Position, ViewShed, World};

pub const MAP_WIDTH: usize = 120;
pub const MAP_HEIGHT: usize = 70;

pub struct Map {
    pub tiles: Vec<WorldTile>,
}

/// Renders the world from the player perspective
pub fn render_map(ctx: &mut BTerm, map: &[WorldTile], world: &World) {
    for (_, (viewshed, _)) in world.query::<(&ViewShed, &Player)>().iter() {
        let mut x = 0;
        let mut y = 0;
        for tile in map.iter() {
            if viewshed.visible_tiles.contains(&Point::new(x, y)) {
                ctx.set(x, y, tile.sprite.fg, tile.sprite.bg, tile.sprite.glyph);
            }
            x += 1;
            if x >= MAP_WIDTH {
                x = 0;
                y += 1;
            }
        }
    }
}

/// Converts 2d coords to 1d index
pub fn xy_to_idx(x: usize, y: usize) -> usize {
    x + (y * MAP_WIDTH)
}

/// Converts 2d position to a 1d index
#[allow(dead_code)]
pub fn pos_to_idx(pos: Position) -> usize {
    pos.x + (pos.y * MAP_WIDTH)
}

/// Converts 1d index to 2d coords
#[allow(dead_code)]
pub fn idx_to_pos(idx: usize) -> Position {
    Position {
        x: idx / MAP_WIDTH,
        y: idx % MAP_WIDTH,
    }
}

#[derive(Copy, Clone)]
pub struct WorldTile {
    pub sprite: CharSprite,
    pub is_blocking: bool,
    pub is_transparent: bool,
}

pub fn generate_map() -> Map {
    // create the map for the overworld
    let mut map = Map {
        tiles: vec![wall(); MAP_HEIGHT * MAP_WIDTH],
    };

    // Perlin noise suited for terrain
    let mut noise = FastNoise::seeded(69);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(6);
    noise.set_fractal_gain(0.05);
    noise.set_fractal_lacunarity(0.7);
    noise.set_frequency(1.9);

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let mut perlin_value = noise.get_noise(x as f32, y as f32);
            perlin_value = (perlin_value + 1.0) * 0.5;

            if perlin_value > 0.4 {
                map.tiles[xy_to_idx(x, y)] = floor();
            }
        }
    }

    map
}


