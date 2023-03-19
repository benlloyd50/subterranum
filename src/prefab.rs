/*  The goal workflow
    1. Make the prefab in RexPaint
    2. Batch load all the prefabs being used
    3. In worldgen, prefabs may be used by the methods
 */

use bracket_terminal::prelude::{XpFile, BLACK};
use crate::{map::WorldTile, tiles::{wall_stone, floor_grass}, actor::CharSprite};


pub struct Prefab {
    pub structure: Vec<WorldTile>,
    pub width: usize,
    pub height: usize,
}

pub fn xy_to_idx(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}


pub fn load_rex_room() -> Prefab {
    let xp_file = XpFile::from_resource("../resources/rex/cave_entrance.xp").unwrap();
    let mut prefab: Vec<WorldTile> = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for layer in &xp_file.layers {
        // If there is more than 1 layer than the prefab will only have the contents of the last layer viewed
        prefab = vec![wall_stone(); layer.width * layer.height];
        width = layer.width;
        height = layer.height;

        for y in 0..layer.height {
            for x in 0..layer.width {
                let cell = layer.get(x, y).unwrap();
                let idx = xy_to_idx(x, y, width);

                match (cell.ch as u8) as char {
                    '.' => prefab[idx] = floor_grass(), // space
                    '#' => prefab[idx] = wall_stone(), // #
                    'P'=> prefab[idx] = {
                        println!("found player spawn");
                        WorldTile {
                            sprite: CharSprite::new('P', BLACK, None),
                            is_blocking: true,
                            is_transparent: false,
                            destructible: crate::map::Destructible::Unbreakable,
                        } 
                    },
                    _ => {
                        println!("didn't match {}", (cell.ch as u8) as char);
                    }
                }
            }
        }
    }

    Prefab { structure: prefab, width, height }
}