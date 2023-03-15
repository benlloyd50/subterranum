use rand::random;
use std::fmt;

use hecs::World;

use crate::{
    actor::{try_move, Position},
    fov::ViewShed,
    map::Map,
    messagelog::Message,
};

pub fn handle_monster_turns(world: &mut World, map: &mut Map, msg_log: &mut Vec<Message>) {
    for (_, (mut pos, view, breed)) in world
        .query::<(&mut Position, &mut ViewShed, &Breed)>()
        .iter()
    {
        let mut new_pos = pos.clone();
        match random::<u8>() % 4 {
            0 => {
                new_pos.x += 1;
            }
            1 => {
                new_pos.x = new_pos.x.saturating_sub(1);
            }
            2 => {
                new_pos.y += 1;
            }
            3 => {
                new_pos.y = new_pos.y.saturating_sub(1);
                msg_log.push(Message::new(format!("{}: A freaking 3 again?", breed.name)));
            }
            _ => {}
        }
        try_move(map, new_pos, &mut pos, view);
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
