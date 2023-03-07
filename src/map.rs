/// Map.rs is the map generation code and data structures to hold information about the map
use crate::{tiles::*, CharSprite, Position, Player};
use bracket_terminal::prelude::{BTerm, Point};
use crate::{World, ViewShed};

pub const MAP_WIDTH: usize = 120;
pub const MAP_HEIGHT: usize = 70;

pub struct Map {
    pub tiles: Vec<Tile>,
}

// Renders the world from the player perspective
pub fn render_map(ctx: &mut BTerm, map: &[Tile], world: &World) {
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

// Converts 2d coords to 1d index
pub fn xy_to_idx(x: usize, y: usize) -> usize {
    x + (y * MAP_WIDTH)
}

// Converts 2d position to a 1d index
#[allow(dead_code)]
pub fn pos_to_idx(pos: Position) -> usize {
    pos.x + (pos.y * MAP_WIDTH)
}

// Converts 1d index to 2d coords
#[allow(dead_code)]
pub fn idx_to_pos(idx: usize) -> Position {
    Position {
        x: idx / MAP_WIDTH,
        y: idx % MAP_WIDTH,
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub sprite: CharSprite,
    pub is_blocking: bool,
    pub is_transparent: bool,
}

pub fn generate_overworld_map() -> Map {
    // create the map for the overworld
    let mut map = Map {
        tiles: vec![
            Tile {
                is_blocking: true,
                sprite: wall(),
                is_transparent: false,
            };
            MAP_HEIGHT * MAP_WIDTH
        ],
    };

    // make a room
    for x in 1..20 {
        for y in 1..15 {
            if let Some(tile) = map.tiles.get_mut(xy_to_idx(x, y)) {
                *tile = Tile {
                    is_blocking: false,
                    is_transparent: true,
                    sprite: floor(),
                };
            }
        }
    }

    map
}

// Finds all tiles that are connected so the others may be filtered
#[allow(dead_code)]
fn flood_fill(map: Vec<Tile>, start_pos: Position) -> Vec<Position> {
    let mut stack = vec![start_pos];
    let mut visited = vec![false; map.len()];
    visited[pos_to_idx(start_pos)] = true;

    while let Some(pos) = stack.pop() {
        for neighbor in get_neighbors(pos) {
            let idx = pos_to_idx(neighbor);
            if !visited[idx] && map[idx].is_blocking {
                visited[idx] = true;
                stack.push(neighbor);
            }
        }
    }

    visited
        .iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .map(|(i, _)| idx_to_pos(i))
        .collect()
}

// gets the 8 surrounding neighbors of a grid based position
fn get_neighbors(from: Position) -> Vec<Position> {
    let mut neighbors = vec![];

    for x in from.x.saturating_sub(1)..=from.x + 1 {
        for y in from.y.saturating_sub(1)..=from.y + 1 {
            if x >= MAP_WIDTH || y >= MAP_HEIGHT || (x == from.x && y == from.y) {
                continue;
            }

            neighbors.push(Position::new(x, y));
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        assert_eq!(
            //top left corner
            get_neighbors(Position { x: 0, y: 0 }),
            vec![
                Position::new(0, 1),
                Position::new(1, 0),
                Position::new(1, 1)
            ]
        );
        assert_eq!(
            // bottom right corner
            get_neighbors(Position { x: 119, y: 69 }),
            vec![
                Position::new(118, 68),
                Position::new(118, 69),
                Position::new(119, 68),
            ]
        );
    }
}
