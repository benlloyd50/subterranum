use crate::actor::Position;
use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::prefab::{load_rex_room, xy_to_idx};
use crate::tiles::*;
use bracket_noise::prelude::*;
use bracket_pathfinding::prelude::Point;
use bracket_random::prelude::*;
use bracket_terminal::prelude::to_cp437;
use std::collections::VecDeque;


pub fn generate_map(seed: u64) -> (Map, Position) {
    let width = 100;
    let height = 70;

    let mut player_spawn = Position::new(0, 0);
    // create the map for the overworld
    let mut map = Map {
        tiles: vec![wall_stone(); width * height],
        visible: vec![false; width * height],
        discovered: vec![false; width * height],
        width,
        height,
        depth: 0,
    };

    // let mut rng = RandomNumberGenerator::seeded(seed);
    create_caverns(&mut map, seed);
    if map.depth == 0 {
        player_spawn = create_entrance(&mut map, seed);
    }


    // brush_spawn(&mut map, &mut rng);

    (map, player_spawn)
}

fn create_caverns(map: &mut Map, seed: u64) {
    let cave_noise = cave_perlin(seed);

    for x in 1..map.width-1 {
        for y in 1..map.height-1 {
            let mut perlin_value = cave_noise.get_noise(x as f32 / 64., y as f32 / 64.);
            perlin_value = (perlin_value + 1.0) * 0.5;

            let idx = map.xy_to_idx(x, y);
            if perlin_value > 0.65 {
                map.tiles[idx] = floor_stone();
            }
        }
    }
}

fn create_entrance(map: &mut Map, _seed: u64) -> Position {
    // Basically want to pick a side of the map and place something down
    let starting_x = 10;
    let starting_y = 0;

    let entrance_prefab = load_rex_room();
    let width = entrance_prefab.width;

    let mut player_spawn = Position::new(0, 0);

    for i in starting_x..starting_x + entrance_prefab.width {
        for j in starting_y..starting_y + entrance_prefab.height {
            let idx = map.xy_to_idx(i, j);
            let prefab_tile = entrance_prefab.structure[xy_to_idx(i - 10, j, width)];

            if prefab_tile.sprite.glyph == to_cp437('P') {
                player_spawn = map.idx_to_pos(idx);
                println!("{:?}", player_spawn);
                map.tiles[idx] = floor_stone();
                continue;
            }

            map.tiles[idx] = prefab_tile;
        }
    }

    player_spawn
}

/// Spawns multiple brushes
#[allow(dead_code)]
fn brush_spawn(map: &mut Map, rng: &mut RandomNumberGenerator) {
    // Get 4 starting(breeding) points
    let starting_points = get_spaced_points(10, map, rng);
    for point in starting_points {
        let mut breeding = VecDeque::new();
        breeding.push_front((point, 0));

        let mut lifetimes = 75;
        while let Some((breeder, priority)) = get_priority(&mut breeding) {
            let idx = breeder.to_index(map.width);
            if map.tiles[idx].is_blocking {
                // skip blocking tiles to prevent brush in a rock or something
                continue;
            }

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

    for i in 0..num_points {
        let x: i32 = rng.range(leftx, rightx);
        let y: i32 = rng.range(topy, bottomy);

        let potential = Point::new(x, y);
        if !map.tiles[potential.to_index(map.width)].is_blocking {
            spaced_points.push(potential);
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

fn cave_perlin(seed: u64) -> FastNoise {
    let mut noise = FastNoise::seeded(seed);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(2);
    noise.set_fractal_gain(0.02);
    noise.set_fractal_lacunarity(0.5);
    noise.set_frequency(2.5);

    noise
}
