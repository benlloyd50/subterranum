/* Map.rs is the map generation code and data structures to hold information about the map
 */
use bracket_noise::prelude::*;
use bracket_terminal::prelude::{BTerm, Point};
use rand::Rng;

use crate::{tiles::*, CharSprite, Position};

pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 70;

pub struct Map {
    pub tiles: Vec<WorldTile>,
    pub visible: Vec<bool>,
    pub discovered: Vec<bool>,
}

/// Renders the world from the player perspective
pub fn render_map(ctx: &mut BTerm, map: &Map) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.tiles.iter() {
        if map.visible[xy_to_idx(x, y)] {
            ctx.set(x, y, tile.sprite.fg, tile.sprite.bg, tile.sprite.glyph);
        } else if map.discovered[xy_to_idx(x, y)]{
            ctx.set(x, y, tile.sprite.fg.to_greyscale(), tile.sprite.bg.to_greyscale(), tile.sprite.glyph);
        }

        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}

/// Converts 2d coords to 1d index
pub fn xy_to_idx(x: usize, y: usize) -> usize {
    x + (y * MAP_WIDTH)
}

/// Converts 2d position to a 1d index
pub fn point_to_idx(point: Point) -> usize {
    point.x as usize + (point.y as usize * MAP_WIDTH)
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
        tiles: vec![wall_stone(); MAP_HEIGHT * MAP_WIDTH],
        visible: vec![false; MAP_WIDTH * MAP_HEIGHT],
        discovered: vec![false; MAP_WIDTH * MAP_HEIGHT]
    };

    // Perlin noise suited for terrain
    let terrain_noise = terrain_perlin(69);

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let mut perlin_value = terrain_noise.get_noise(x as f32 / 64., y as f32 / 64.);
            perlin_value = (perlin_value + 1.0) * 0.5;

            if perlin_value < 0.4 {
                map.tiles[xy_to_idx(x, y)] = deep_water();
            } else if perlin_value < 0.75 {
                map.tiles[xy_to_idx(x, y)] = floor_stone();
            } else {
                map.tiles[xy_to_idx(x, y)] = wall_stone();
            }
        }
    }

    map
}

fn brush_spawn(map : &mut Map) {
    

}

fn terrain_perlin(seed: u64) -> FastNoise {
    let mut noise = FastNoise::seeded(seed);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(6);
    noise.set_fractal_gain(0.05);
    noise.set_fractal_lacunarity(0.7);
    noise.set_frequency(1.9);

    noise
}

