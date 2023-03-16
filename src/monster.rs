use bracket_pathfinding::prelude::{a_star_search, DistanceAlg, Point};
use rand::random;
use std::fmt;

use hecs::{With, World};

use crate::{
    actor::{try_move, Player, Position},
    fov::ViewShed,
    map::Map,
    messagelog::Message,
};

pub fn handle_monster_turns(world: &mut World, map: &mut Map, msg_log: &mut Vec<Message>) {
    for (_, player_pos) in world.query::<With<&Position, &Player>>().iter() {
        let player_idx = map.xy_to_idx(player_pos.x, player_pos.y);
        let player_point = Point::new(player_pos.x, player_pos.y);

        for (_, (mut pos, mut view, breed)) in world.query::<(&mut Position, &mut ViewShed, &Breed)>().iter() {
            let dist_to_player = DistanceAlg::Pythagoras.distance2d(player_point, Point::new(pos.x, pos.y));

            if dist_to_player < 1.5 {
                msg_log.push(Message::new(format!("{}: Poke", breed.name)));
                continue;
            }

            if view.visible_tiles.contains(&player_point) {
                let path = a_star_search(map.xy_to_idx(pos.x, pos.y), player_idx, map);
                if path.success && path.steps.len() > 1 {

                    let next_pos = map.idx_to_pos(path.steps[1]);
                    if try_move(map, next_pos, &mut pos, &mut view) {
                        continue;
                    }
                }
            }

            let mut new_pos = pos.clone();
            match random::<u8>() % 4 {
                0 => { new_pos.x += 1; }
                1 => { new_pos.x = new_pos.x.saturating_sub(1); }
                2 => { new_pos.y += 1; }
                3 => { new_pos.y = new_pos.y.saturating_sub(1); }
                _ => {}
            }
            msg_log.push(Message::new(format!("{}: I'm moving freely", breed.name)));
            try_move(map, new_pos, &mut pos, view);
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
