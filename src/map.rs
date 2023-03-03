use bracket_terminal::prelude::{to_cp437, BLACK, BROWN1, BROWN4, RGB, WHITESMOKE};

/// Map.rs is the map generation code and data structures to hold information about the map
use crate::{CharSprite, Position};

pub const MAP_WIDTH: usize = 120;
pub const MAP_HEIGHT: usize = 70;

pub struct Map {
    pub overworld: Vec<Tile>,
}

// Converts 2d coords to 1d index
pub fn xy_to_idx(x: usize, y: usize) -> usize {
    x + (y * MAP_WIDTH)
}

// Converts a 2d position to a 1d index
#[allow(dead_code)]
pub fn pos_to_idx(pos: Position) -> usize {
    pos.x + (pos.y * MAP_WIDTH)
}

// Converts 1d index to 2d coords
#[allow(dead_code)]
pub fn idx_to_xy(idx: usize) -> Position {
    Position {
        x: idx / MAP_WIDTH,
        y: idx % MAP_WIDTH,
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub is_blocking: bool,
    pub sprite: CharSprite,
}

pub fn generate_overworld_map() -> Map {
    let mut map = Map {
        overworld: vec![
            Tile {
                is_blocking: true,
                sprite: wall()
            };
            MAP_HEIGHT * MAP_WIDTH
        ],
    };

    for x in 1..20 {
        for y in 1..15 {
            if let Some(tile) = map.overworld.get_mut(xy_to_idx(x, y)) {
                *tile = Tile {
                    is_blocking: false,
                    sprite: floor(),
                };
            }
        }
    }

    map
}

// fn flood_fill(map: Vec<Tile>, start_pos: Position) -> Vec<Position> {
//     let mut stack = vec![start_pos];
//     let mut visited = vec![false; map.len()];
//     visited[pos_to_idx(start_pos)] = true;
//
//     while let Some(pos) = stack.pop() {
//         for neighbor in self.get_neighbors(pos) {
//             let idx = self.cell_idx(neighbor);
//             if !visited[idx] && self.cells[idx].is_wall {
//                 visited[idx] = true;
//                 stack.push(neighbor);
//             }
//         }
//     }
//
//     visited.iter()
//         .enumerate()
//         .filter(|(_, &v)| v)
//         .map(|(i, _)| self.cells[i].pos)
//         .collect()
// }

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

fn wall() -> CharSprite {
    CharSprite {
        glyph: to_cp437('#'),
        fg: RGB::named(BROWN1),
        bg: RGB::named(BROWN4),
    }
}

fn floor() -> CharSprite {
    CharSprite {
        glyph: to_cp437('.'),
        fg: RGB::named(WHITESMOKE),
        bg: RGB::named(BLACK),
    }
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
