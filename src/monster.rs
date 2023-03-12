use rand::random;
use std::fmt;

use hecs::With;

use crate::{actor::{Position, try_move}, State, fov::ViewShed};

pub fn handle_monster_turns(state: &mut State) {
    for (_, (mut pos, view)) in state.world.query::<With<(&mut Position, &mut ViewShed), &Breed>>().iter() {
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
            }
            _ => {}
        }
        try_move(&state.map, new_pos, &mut pos, view);
    }
}

/// General info about the type of monster/creature
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
