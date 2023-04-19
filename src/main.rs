use bracket_terminal::prelude::*;
use hecs::*;
use rand::{seq::SliceRandom, Rng};
use std::{collections::HashMap, fs};

mod data_read;
use data_read::{named_living_builder, ENTITY_DB};
mod gui;
mod map;
mod menu;
mod messagelog;
mod monster;
mod prefab;
mod worldgen;
use map::Map;
use worldgen::generate_map;
mod actor;
mod fov;
mod item;
use actor::{CharSprite, Player, Position};
mod config;
mod input;
mod map_scanning;
mod state;

use crate::{data_read::load_data_for_entities, menu::MenuIndex, state::{State, RunState}, messagelog::Message, config::Config};

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/RDE.png");
bracket_terminal::embedded_resource!(CAVE_ENTRANCE, "../resources/rex/cave_entrance.xp");
bracket_terminal::embedded_resource!(INTRO_SCREEN, "../resources/rex/intro_screen.xp");
bracket_terminal::embedded_resource!(MENU_OPTIONS, "../resources/rex/options_box.xp");

fn main() -> BError {
    load_data_for_entities();

    // Reads in a config file to setup the game
    let contents: String = fs::read_to_string("resources/config.toml")?;
    let config: Config = toml::from_str(&contents).unwrap();

    bracket_terminal::link_resource!(CAVE_ENTRANCE, "../resources/rex/cave_entrance.xp");
    bracket_terminal::link_resource!(INTRO_SCREEN, "../resources/rex/intro_screen.xp");
    bracket_terminal::link_resource!(MENU_OPTIONS, "../resources/rex/options_box.xp");

    // Setup terminal renderer
    bracket_terminal::link_resource!(TILE_FONT, "resources/RDE.png");
    let context = BTermBuilder::new()
        .with_title("Terra Incognita [ALPHA]")
        .with_fullscreen(config.fullscreen)
        .with_dimensions(config.screensize_x, config.screensize_y)
        .with_tile_dimensions(config.font_size, config.font_size)
        .with_font_bg(
            &config.font_file,
            config.font_size,
            config.font_size,
            RGB::from_u8(255, 0, 255),
        )
        .with_simple_console(config.screensize_x, config.screensize_y, &config.font_file)
        .build()?;

    let mut world = World::new();
    let gs = if config.dev_mode {
        // For dev purposes, we can skip the main menu
        let map = start_new_game(&mut world, config.world_seed);
        State {
            world,
            map,
            runstate: RunState::InGame,
            config,
            message_log: vec![
                Message::new("Welcome to Terra Incognita".to_string()),
                Message::new("This is an alpha build from March 2023".to_string()),
            ],
            turn_counter: 0,
            generated_maps: HashMap::new(),
        }
    } else {
        State {
            world,
            map: Map::empty(),
            runstate: RunState::MainMenu(MenuIndex(0)),
            config,
            message_log: vec![
                Message::new("Welcome to Terra Incognita".to_string()),
                Message::new("This is an alpha build from March 2023".to_string()),
            ],
            turn_counter: 0,
            generated_maps: HashMap::new(),
        }
    };

    main_loop(context, gs)
}

/// Creates a new map and setups world for the start of a fresh run
pub fn start_new_game(world: &mut World, seed: u64) -> Map {
    let (mut map, player_start) = generate_map(seed, 0);
    furnish_map(world, &mut map, player_start);
    map
}

fn furnish_map(world: &mut World, map: &mut Map, player_pos: Position) {
    let player_builder = named_living_builder(&ENTITY_DB.lock().unwrap(), "Player", player_pos);
    if let Some(mut pb) = player_builder {
        world.spawn(pb.build());
    }

    add_beings_to_rooms(world, map);
}

fn add_beings_to_rooms(world: &mut World, map: &mut Map) {
    let monster_spawns_per_room = 5;

    let beings = vec!["Centipede", "Mole", "Star Nosed Mole"];
    for room in map.rooms.iter() {
        for _ in 0..monster_spawns_per_room {
            let chance: f32 = rand::thread_rng().gen();
            if  chance > 0.8 {
                continue;
            }

            let being_pos = room.get_random_point();
            let being_name = beings.choose(&mut rand::thread_rng()).unwrap();
            let e_builder = named_living_builder(&ENTITY_DB.lock().unwrap(), being_name, Position(being_pos));
            if let Some(mut eb) = e_builder {
                world.spawn(eb.build());
            }
        }
    }
}
