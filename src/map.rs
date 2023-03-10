/* Map.rs is the map generation code and data structures to hold information about the map
 */
use std::collections::VecDeque;
use bracket_noise::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::BTerm;
use bracket_terminal::prelude::Point;

use crate::{tiles::*, CharSprite};

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
pub fn idx_to_point(idx: usize) -> Point {
    Point {
        x: (idx / MAP_WIDTH) as i32,
        y: (idx % MAP_HEIGHT) as i32,
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
        discovered: vec![false; MAP_WIDTH * MAP_HEIGHT],
    };

    // Perlin noise suited for terrain
    let terrain_noise = terrain_perlin(69);

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let mut perlin_value = terrain_noise.get_noise(x as f32 / 64., y as f32 / 64.);
            perlin_value = (perlin_value + 1.0) * 0.5;

            if perlin_value < 0.3 {
                map.tiles[xy_to_idx(x, y)] = deep_water();
            } else if perlin_value < 0.75 {
                map.tiles[xy_to_idx(x, y)] = floor_stone();
            } else {
                map.tiles[xy_to_idx(x, y)] = wall_stone();
            }
        }
    }

    brush_spawn(&mut map);

    map
}

/// Spawns multiple brushes
fn brush_spawn(map: &mut Map) {

    // Get 4 starting(breeding) points
    let mut rng = RandomNumberGenerator::new();
    let starting_points = get_spaced_points(10, map, &mut rng);
    for point in starting_points {
        let mut breeding = VecDeque::new();
        breeding.push_front((point,0));

        let mut lifetimes = 75;
        let mut planted = 0;
        while let Some((breeder, priority)) = get_priority(&mut breeding) {
            let idx = point_to_idx(breeder);
            if map.tiles[idx].is_blocking {
                // skip blocking tiles to prevent brush in a rock or something
                continue;
            }
            planted += 1;
            map.tiles[idx] = lush_brush();
            for neighbor in get_neighbors(breeder) {
                if rng.rand::<f32>() < 0.4 {
                    breeding.push_back((neighbor, priority + 1));
                }
            }

            lifetimes -= 1;
            if lifetimes <= 0 {
                break;
            }
        }
        println!("{:#?}", planted);

    }
}

/// Gets spaced random points by looking through 4 equal perimeter rectangles inside of 
/// the larger rectangle that is the map
fn get_spaced_points(num_points: u32, map: &Map, rng: &mut RandomNumberGenerator) -> Vec<Point> {
    let mut spaced_points = vec![];

    let mut leftx = 0;
    let mut rightx = MAP_WIDTH as i32 / 2;
    let mut topy = 0;
    let mut bottomy = MAP_HEIGHT as i32 / 2;

    let mut amt = num_points;
    for i in 0..amt {
        let x: i32 = rng.range(leftx, rightx);
        let y: i32 = rng.range(topy, bottomy);

        let potential = Point::new(x, y);
        if !map.tiles[point_to_idx(potential)].is_blocking {
            spaced_points.push(potential);
        } else {
            amt += 1
        } 

        match i % 4{
            0 => {
                leftx += MAP_WIDTH as i32 / 2;
                rightx += MAP_WIDTH as i32 / 2;
            }, 1 => {
                topy += MAP_HEIGHT as i32 / 2;
                bottomy += MAP_HEIGHT as i32 / 2;
            },  2 => {
                leftx -= MAP_WIDTH as i32 / 2;
                rightx -= MAP_WIDTH as i32 / 2;
            }, 3 =>  {
                topy -= MAP_HEIGHT as i32 / 2;
                bottomy -= MAP_HEIGHT as i32 / 2;
            },
            _ => unreachable!(),
        }     
    }

    spaced_points
}

fn get_priority(vec: &mut VecDeque<(Point, i32)>) -> Option<(Point, i32)> {
    vec.make_contiguous().sort_by(|x, y| x.1.cmp(&y.1));
    vec.pop_front()
}

/// Gets the 8 neighboring tiles to a point
fn get_neighbors(point: Point) -> Vec<Point> {
    let mut neighbors = vec![];

    for x in (point.x - 1)..=(point.x + 1) {
        for y in (point.y - 1)..=(point.y + 1) {
            if is_valid_neighbor(x, y, point) {
                neighbors.push(Point::new(x, y));
            }
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_neighbor() {
        let neighbors = vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 0),
            Point::new(1, 2),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ];
        assert_eq!(get_neighbors(Point::new(1, 1)), neighbors);
    }
}

/// A valid neighbor is a point inside the map bounds and not the original branching point
fn is_valid_neighbor(x: i32, y: i32, starting: Point) -> bool {
    x < MAP_WIDTH as i32
        && y < MAP_HEIGHT as i32
        && y >= 0
        && x >= 0
        && Point::new(x, y).ne(&starting)
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
