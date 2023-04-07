use crate::actor::Position;
use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::prefab::{load_rex_room, xy_to_idx};
use crate::tiles::*;
use bracket_noise::prelude::*;
use bracket_pathfinding::prelude::Point;
use bracket_random::prelude::*;
use bracket_terminal::prelude::to_cp437;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct WorldRoom {
    pub tiles: Vec<Point>,
}

impl WorldRoom {
    fn new() -> Self {
        Self { tiles: Vec::new() }
    }
}

pub fn generate_map(seed: u64, depth: usize) -> (Map, Position) {
    let width = 100;
    let height = 70;

    // create the map for the overworld
    let mut map = Map {
        tiles: vec![wall_stone(); width * height],
        visible: vec![false; width * height],
        discovered: vec![false; width * height],
        rooms: Vec::new(),
        width,
        height,
        depth,
    };

    let mut rng = RandomNumberGenerator::seeded(seed);
    create_caverns(&mut map, seed);

    let player_spawn = if map.depth == 0 {
        create_entrance(&mut map, &mut rng)
    } else {
        Position::new(0, 0)
    };

    cull_rooms(&mut map);
    remove_small_rooms(&mut map, 10);

    debug_map_rooms(&mut map);

    place_down_stairs(&mut map, &mut rng);

    // brush_spawn(&mut map, &mut rng);

    (map, player_spawn)
}

/// displays each room's floor as a single hex digit 1-f, skips any rooms past the 16th for now
/// this affects the map by setting each ground tile to a number, maybe this could be done on the
/// rendering side instead
fn debug_map_rooms(map: &mut Map) {
    let mut room_num = 0;
    let symbols = (b'0'..=b'9')
        .chain(b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(char::from)
        .collect::<Vec<_>>();

    for room in &mut map.rooms {
        for tile in &room.tiles {
            map.tiles[tile.to_index(map.width)].sprite.glyph = to_cp437(symbols[room_num]);
        }

        room_num += 1;
    }
}

fn place_down_stairs(map: &mut Map, rng: &mut RandomNumberGenerator) {
    let room_idx = rng.range(0, map.rooms.len());
    let room_pos_idx = rng.range(0, map.rooms[room_idx].tiles.len());
    let stair_pos = map.rooms[room_idx].tiles[room_pos_idx].to_index(map.width);
    map.tiles[stair_pos] = down_stairs();
}

fn create_caverns(map: &mut Map, seed: u64) {
    let cave_noise = cave_perlin(seed);

    for x in 1..map.width - 1 {
        for y in 1..map.height - 1 {
            let mut perlin_value = cave_noise.get_noise(x as f32 / 64., y as f32 / 64.);
            perlin_value = (perlin_value + 1.0) * 0.5;

            let idx = map.xy_to_idx(x, y);
            if perlin_value > 0.6 && perlin_value < 0.9 {
                map.tiles[idx] = floor_stone();
            }
        }
    }
}

/// Goes around the edge of the cave and makes the edges unbreakable
#[allow(dead_code)]
fn seal_cavern(map: &mut Map) {
    for cx in 0..MAP_WIDTH {
        let idx = map.xy_to_idx(cx, 0);
        map.tiles[idx] = wall_adamnatite();
        let idx = map.xy_to_idx(cx, map.height - 1);
        map.tiles[idx] = wall_adamnatite();
    }

    for cy in 0..MAP_HEIGHT {
        let idx = map.xy_to_idx(0, cy);
        map.tiles[idx] = wall_adamnatite();
        let idx = map.xy_to_idx(map.width - 1, cy);
        map.tiles[idx] = wall_adamnatite();
    }
}

fn create_entrance(map: &mut Map, rng: &mut RandomNumberGenerator) -> Position {
    let entrance_prefab = load_rex_room("cave_entrance");

    let starting_x = rng.range(10, map.width - entrance_prefab.width);
    let starting_y = 0;

    let mut player_spawn = Position::new(0, 0);

    for x in starting_x..starting_x + entrance_prefab.width {
        for y in starting_y..starting_y + entrance_prefab.height {
            let idx = map.xy_to_idx(x, y);
            let prefab_idx = xy_to_idx(x - starting_x, y, entrance_prefab.width);
            let prefab_tile = entrance_prefab.structure[prefab_idx];

            // checks for a special player spawn tile in the prefab
            if prefab_tile.sprite.glyph == to_cp437('P') {
                player_spawn = map.idx_to_pos(idx);
                map.tiles[idx] = floor_grass();
                continue;
            }

            map.tiles[idx] = prefab_tile;
        }
    }

    player_spawn
}

/// Spawns multiple brushes on floor tiles
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

/// scans the map and collects all "bodies" of land as a room, if every room is connected then
/// there will only be one room
fn cull_rooms(map: &mut Map) {
    let mut visited = vec![false; map.width * map.height];

    for i in 0..map.tiles.len() {
        let tile = &mut map.tiles[i];
        if visited[i] {
            continue;
        }
        if !tile.sprite.eq(floor_stone().sprite) {
            visited[i] = true;
            continue;
        }

        let room = flood_fill(map.idx_to_point(i), map, &mut visited);

        map.rooms.push(room);
    }
}

/// Removes rooms that are under a certain size
fn remove_small_rooms(map: &mut Map, min_size: usize) {
    let mut i = 0;
    loop {
        if map.rooms[i].tiles.len() < min_size {
            // remove small rooms
            let room = map.rooms.remove(i);
            for pt in &room.tiles {
                map.tiles[pt.to_index(map.width)] = wall_stone();
            }
        } else {
            i += 1;
            if i >= map.rooms.len() {
                break;
            }
        }
    }
}

/// Checks neighbors and finds all points within a single continuous room
fn flood_fill(starting: Point, map: &Map, visited: &mut Vec<bool>) -> WorldRoom {
    let mut room = WorldRoom::new();
    let mut unvisited = vec![starting];

    while let Some(pt) = unvisited.pop() {
        // visit state, get neighbors, put on stack if they are floor, mark visited so we skip it next time
        let idx = pt.to_index(map.width);
        visited[idx] = true;
        room.tiles.push(pt);

        let neighbors = get_neighbors(pt);
        if neighbors.len() == 0 {
            println!("didn't find any neighbors");
            continue;
        }

        for neighbor in neighbors {
            let neighbor_idx = neighbor.to_index(map.width);
            if visited[neighbor_idx] {
                continue;
            }

            if map.tiles[neighbor_idx].sprite.eq(floor_stone().sprite) {
                unvisited.push(neighbor);
            } else {
                visited[neighbor_idx] = true; // mark any tile that isn't in the room nor walkable as visited
            }
        }
    }

    room
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
