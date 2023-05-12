use serde::{Deserialize, Serialize};
use std::fs;

use crate::{
    actor::{Player, Position},
    add_player_to_room,
    config::Config,
    furnish_map,
    map::Map,
    state::State,
    worldgen::cull_destructibles,
};

#[derive(Deserialize, Serialize)]
struct GameData {
    maps: Vec<Map>,
    depths: Vec<usize>,
    last_depth: usize,
    last_pos: Position,
    seed: u64,
}

impl GameData {
    fn new() -> Self {
        Self {
            maps: Vec::new(),
            depths: Vec::new(),
            last_depth: 0,
            last_pos: Position::new(0, 0),
            seed: 0,
        }
    }
}

/// Saves game data to a file currently set to player.sav, in the future there will be a way to get a name
pub fn save_game(state: &mut State) {
    let data = generate(state);
    let save_name = "player";

    let data_json = serde_json::to_string(&data).unwrap();
    let path = format!("./saves/{save_name}.sav");
    match fs::write(&path, data_json) {
        Ok(..) => {
            println!("Successful save to {}", path);
        }
        Err(err) => {
            println!("Error while saving {}, can't recover yet", err);
        }
    }
}

/// Generates the save data from state for serialization
fn generate(state: &mut State) -> GameData {
    // make sure the most recent generated map is updated
    state.generated_maps.insert(state.map.depth, state.map.clone());

    let mut data = GameData::new();

    data.seed = state.config.world_seed;
    data.last_depth = state.map.depth;

    // Collect all the maps the player has visited as it may have destroyed terrain so we couldn't regenerate it
    for (depth, map) in state.generated_maps.iter() {
        data.depths.push(*depth);
        data.maps.push(map.clone());
    }

    if let Some((_, (_, pos))) = state.world.query::<(&Player, &Position)>().iter().next() {
        data.last_pos = pos.clone();
    }

    data
}

/// Starts a new game from a defined saved game
pub fn start_load_game(config: Config) -> State {
    let load_data = load_state_from_save();
    let width = 100;
    let height = 70;

    let mut load_state = State::new(&config);
    for (map, depth) in load_data.maps.iter().zip(load_data.depths.iter()) {
        load_state.generated_maps.insert(*depth, map.clone());
    }

    for (_, map) in load_state.generated_maps.iter_mut() {
        map.beings = vec![None; width * height];
        map.destructibles = vec![None; width * height];
    }

    load_state.map = match load_state.generated_maps.get(&load_data.last_depth) {
        Some(map) => map.clone(),
        None => panic!("Map could not be found for the last recorded depth"),
    };

    generate_content(&mut load_state, load_data.last_pos);

    load_state
}

/// Loads data that is algorithmically creatable
fn generate_content(state: &mut State, player_pos: Position) {
    add_player_to_room(&mut state.world, player_pos);
    furnish_map(&mut state.world, &mut state.map);
    cull_destructibles(&mut state.map);
}

/// Attempts to loads static data we can't regenerate
fn load_state_from_save() -> GameData {
    let name = "player";
    let data_json = match fs::read_to_string(format!("./saves/{name}.sav")) {
        Ok(json) => json,
        Err(e) => panic!("error loading save file for \"{name}\" with {e}"),
    };

    let game_data: GameData = match serde_json::from_str(&data_json) {
        Ok(data) => data,
        Err(e) => panic!("error deserialzing json string for \"{name}\" with {e}"),
    };

    game_data
}
