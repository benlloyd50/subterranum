use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::tiles::*;
use bracket_noise::prelude::*;
use bracket_pathfinding::prelude::Point;
use bracket_random::prelude::*;
use std::collections::VecDeque;

pub fn generate_map(seed: u64) -> Map {
    let width = 100;
    let height = 70;

    // create the map for the overworld
    let mut map = Map {
        tiles: vec![wall_stone(); width * height],
        visible: vec![false; width * height],
        discovered: vec![false; width * height],
        width,
        height,
    };

    let mut rng = RandomNumberGenerator::seeded(seed);
    // Perlin noise suited for terrain
    let terrain_noise = terrain_perlin(seed);

    for x in 0..map.width {
        for y in 0..map.height {
            let mut perlin_value = terrain_noise.get_noise(x as f32 / 64., y as f32 / 64.);
            perlin_value = (perlin_value + 1.0) * 0.5;

            let idx = map.xy_to_idx(x, y);
            if perlin_value < 0.3 {
                map.tiles[idx] = deep_water();
            } else if perlin_value < 0.75 {
                map.tiles[idx] = floor_stone();
            } else {
                map.tiles[idx] = wall_stone();
            }
        }
    }

    brush_spawn(&mut map, &mut rng);

    map
}

/// Spawns multiple brushes
fn brush_spawn(map: &mut Map, rng: &mut RandomNumberGenerator) {
    // Get 4 starting(breeding) points
    let starting_points = get_spaced_points(10, map, rng);
    for point in starting_points {
        let mut breeding = VecDeque::new();
        breeding.push_front((point, 0));

        let mut lifetimes = 75;
        let mut planted = 0;
        while let Some((breeder, priority)) = get_priority(&mut breeding) {
            let idx = map.point_to_idx(breeder);
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
    let mut rightx = map.width as i32 / 2;
    let mut topy = 0;
    let mut bottomy = map.height as i32 / 2;

    let mut amt = num_points;
    for i in 0..amt {
        let x: i32 = rng.range(leftx, rightx);
        let y: i32 = rng.range(topy, bottomy);

        let potential = Point::new(x, y);
        if !map.tiles[map.point_to_idx(potential)].is_blocking {
            spaced_points.push(potential);
        } else {
            amt += 1
        }

        match i % 4 {
            0 => {
                leftx += map.width as i32 / 2;
                rightx += map.width as i32 / 2;
            }
            1 => {
                topy += map.height as i32 / 2;
                bottomy += map.height as i32 / 2;
            }
            2 => {
                leftx -= map.width as i32 / 2;
                rightx -= map.width as i32 / 2;
            }
            3 => {
                topy -= map.height as i32 / 2;
                bottomy -= map.height as i32 / 2;
            }
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

/// A valid neighbor is a point inside the map bounds and not the original branching point
fn is_valid_neighbor(x: i32, y: i32, starting: Point) -> bool {
    x < MAP_WIDTH as i32 && y < MAP_HEIGHT as i32 && y >= 0 && x >= 0 && Point::new(x, y).ne(&starting)
}

#[cfg(test)]
mod tests {
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
