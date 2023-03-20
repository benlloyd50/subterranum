/*  The goal workflow
   1. Make the prefab in RexPaint
   2. Batch load all the prefabs being used
   3. In worldgen, prefabs may be used by the methods
*/

use crate::{
    actor::CharSprite,
    map::WorldTile,
    tiles::{floor_grass, wall_stone},
};
use bracket_terminal::prelude::{XpFile, BLACK};

#[derive(Default)]
pub struct Prefab {
    pub structure: Vec<WorldTile>,
    pub width: usize,
    pub height: usize,
}

pub fn xy_to_idx(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}

/// Loads a rex image file from it's name without extension
/// Returns a prefab with the abstract details of the map
pub fn load_rex_room(rex_file: impl ToString) -> Prefab {
    let file_name = &format!("../resources/rex/{}.xp", rex_file.to_string());
    let xp_file = XpFile::from_resource(file_name).unwrap();

    let mut prefab = Prefab::default();

    for layer in &xp_file.layers {
        // If there is more than 1 layer than the prefab will only have the contents of the last layer viewed
        prefab.structure = vec![wall_stone(); layer.width * layer.height];
        prefab.width = layer.width;
        prefab.height = layer.height;

        for y in 0..layer.height {
            for x in 0..layer.width {
                let cell = layer.get(x, y).unwrap();
                let idx = xy_to_idx(x, y, prefab.width);

                prefab.structure[idx] = match (cell.ch as u8) as char {
                    '.' => floor_grass(), // space
                    '#' => wall_stone(),  // #
                    'P' => WorldTile {
                        sprite: CharSprite::new('P', BLACK, None),
                        is_blocking: true,
                        is_transparent: false,
                        destructible: crate::map::Destructible::Unbreakable,
                    },
                    _ => {
                        println!("{}, {} didn't match {}", x, y, (cell.ch as u8) as char);
                        WorldTile {
                            sprite: CharSprite::new('E', BLACK, None),
                            is_blocking: true,
                            is_transparent: false,
                            destructible: crate::map::Destructible::Unbreakable,
                        }
                    }
                }
            }
        }
    }

    prefab
}
