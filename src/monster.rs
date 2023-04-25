use bracket_pathfinding::prelude::{a_star_search, DistanceAlg};
use rand::random;
use std::cmp::max;

use hecs::With;

use crate::{
    actor::{try_move, MoveResult, Player, Position},
    fov::ViewShed,
    Message, State,
};

pub fn handle_monster_turns(state: &mut State) {
    let width = state.map.width;
    let turn = state.turn_counter;
    if let Some((_, player_pos)) = &mut state.world.query::<With<&Position, &Player>>().iter().next() {
        let player_idx = player_pos.0.to_index(width);

        for (e, (pos, view, breed)) in state.world.query::<(&mut Position, &mut ViewShed, &Breed)>().iter() {
            //TODO: how can i encapsulate this behavior and vary it for different monsters/entities
            let dist_to_player = DistanceAlg::Pythagoras.distance2d(player_pos.0, pos.0);
            if dist_to_player < 1.5 {
                state
                    .message_log
                    .push(Message::new(format!("{}: Poke", breed.name), turn));
                continue;
            }
            let tile_idx = pos.0.to_index(width);

            if view.visible_tiles.contains(&player_pos.0) {
                let path = a_star_search(tile_idx, player_idx, &state.map);
                if path.success && path.steps.len() > 1 {
                    let next_pos = state.map.idx_to_pos(path.steps[1]);
                    match try_move(&mut state.map, &next_pos, pos, view, e) {
                        MoveResult::Acted(_) => continue,
                        _ => {}
                    }
                }
            } else {
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
                try_move(&mut state.map, &new_pos, pos, view, e);
            }
        }
    }
}

/// General info about the type of monster/creature
#[derive(Clone)]
pub struct Breed {
    pub name: String,
    _species: String,
    ai: BeingAI,
}

#[derive(Clone)]
pub enum BeingAI {
    BasicPoke, // Simplest AI being able to wander, follow the player if they are visible, and poke the player
}

impl BeingAI {}

impl Breed {
    pub fn from(name: impl ToString, species: impl ToString, ai: impl ToString) -> Self {
        let ai = match ai.to_string().as_str() {
            "basic" => BeingAI::BasicPoke,
            _ => BeingAI::BasicPoke,
        };
        Self {
            name: name.to_string(),
            _species: species.to_string(),
            ai,
        }
    }
}
