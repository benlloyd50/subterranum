use bracket_pathfinding::prelude::{a_star_search, DistanceAlg};
use rand::random;
use std::{cmp::max, fmt};

use hecs::{With, World};

use crate::{
    actor::{try_move, Player, Position},
    fov::ViewShed,
    map::Map,
    messagelog::Message,
};

pub fn handle_monster_turns(world: &mut World, map: &mut Map, msg_log: &mut Vec<Message>) {
    if let Some((_, player_pos)) = world.query::<With<&Position, &Player>>().iter().next() {
        let player_idx = player_pos.0.to_index(map.width);

        for (_, (pos, view, breed)) in world.query::<(&mut Position, &mut ViewShed, &Breed)>().iter() {
            let dist_to_player = DistanceAlg::Pythagoras.distance2d(player_pos.0, pos.0);

            if dist_to_player < 1.5 {
                msg_log.push(Message::new(format!("{}: Poke", breed.name)));
                continue;
            }

            if view.visible_tiles.contains(&player_pos.0) {
                let path = a_star_search(pos.0.to_index(map.width), player_idx, map);
                if path.success && path.steps.len() > 1 {
                    let next_pos = map.idx_to_pos(path.steps[1]);
                    if try_move(map, next_pos, pos, view) {
                        continue;
                    }
                }
            }

            let mut new_pos = pos.clone();
            match random::<u8>() % 4 {
                0 => {
                    new_pos.0.x += 1;
                }
                1 => {
                    new_pos.0.x = max(new_pos.x() - 1, 0);
                }
                2 => {
                    new_pos.0.y += 1;
                }
                3 => {
                    new_pos.0.y = max(new_pos.y() - 1, 0);
                }
                _ => {}
            }
            msg_log.push(Message::new(format!("{}: I'm moving freely", breed.name)));
            try_move(map, new_pos, pos, view);
        }
    }
}

/// General info about the type of monster/creature
#[allow(dead_code)]
pub struct Breed {
    being_type: MonsterType,
    name: String,
}

impl Breed {
    pub fn new(being_type: MonsterType) -> Self {
        let monster_name = format!("Jeff the {}", being_type);

        Self {
            being_type,
            name: monster_name,
        }
    }
}

/// A high level definition of the type of the monster
#[derive(Debug)]
pub enum MonsterType {
    Centipede,
}

impl fmt::Display for MonsterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
