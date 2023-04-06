use bracket_terminal::prelude::*;
use data_read::named_monster_builder;
use gui::draw_gui;
use hecs::*;
use serde::Deserialize;
use std::{cmp::max, fs, process::exit};

mod data_read;
use data_read::ENTITY_DB;
mod gui;
mod map;
mod menu;
mod messagelog;
mod monster;
mod prefab;
mod worldgen;
use map::{render_map, Map};
use worldgen::generate_map;
mod fov;
use fov::{update_vision, ViewShed};
mod actor;
mod item;
mod tiles;
use actor::{render_entities, try_move, CharSprite, Player, Position};
use menu::run_menu_systems;
use messagelog::Message;
use monster::handle_monster_turns;

use crate::{
    data_read::load_data_for_entities,
    menu::MenuIndex,
};

pub struct State {
    world: World, // Holds all of our entities
    map: Map,     // Holds the tiles to the world
    // player_e: Entity // The player's entity for convienent lookup
    runstate: RunState,
    message_log: Vec<Message>,
    config: Config,
    turn_counter: usize,
}

#[derive(Clone)]
pub enum RunState {
    InGame,
    MainMenu(MenuIndex),
}

impl State {
    /// Systems that are ran every frame, regardless of turn progression
    fn run_continuous_systems(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        update_vision(self);

        render_map(ctx, &self.map, &self.config);
        render_entities(ctx, self);

        draw_gui(ctx, self);
    }

    /// Checks for player's input and runs corresponding action
    /// Returns True if the player's action requires a response
    fn player_input(&mut self, ctx: &mut BTerm) -> bool {
        let mut should_respond = false;
        for (_, (pos, view)) in self
            .world
            .query::<(With<&mut Position, &Player>, &mut ViewShed)>()
            .iter()
        {
            // dest_tile represents the position of something the player will interact with
            let mut dest_pos = pos.clone();
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::W | VirtualKeyCode::K => {
                        dest_pos.0.y = max(dest_pos.y() - 1, 0);
                        should_respond = try_move(&mut self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::S | VirtualKeyCode::J => {
                        dest_pos.0.y += 1;
                        should_respond = try_move(&mut self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::A | VirtualKeyCode::H => {
                        dest_pos.0.x = max(dest_pos.x() - 1, 0);
                        should_respond = try_move(&mut self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::D | VirtualKeyCode::L => {
                        dest_pos.0.x += 1;
                        should_respond = try_move(&mut self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::Y => {
                        dest_pos.0.x = max(dest_pos.x() - 1, 0);
                        dest_pos.0.y = max(dest_pos.y() - 1, 0);
                        should_respond = try_move(&mut self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::U => {
                        dest_pos.0.x += 1;
                        dest_pos.0.y = max(dest_pos.y() - 1, 0);
                        should_respond = try_move(&mut self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::N => {
                        dest_pos.0.x = max(dest_pos.x() - 1, 0);
                        dest_pos.0.y += 1;
                        should_respond = try_move(&mut self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::M => {
                        dest_pos.0.x += 1;
                        dest_pos.0.y += 1;
                        should_respond = try_move(&mut self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::Space => {
                        // A waiting action
                        should_respond = true;
                    }
                    VirtualKeyCode::Escape => exit(0),
                    _ => {}
                }
            }
        }
        should_respond
    }

    /// Response systems are ran after a player inputs something that progresses a turn
    fn run_response_systems(&mut self) {
        handle_monster_turns(&mut self.world, &mut self.map, &mut self.message_log);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut newstate = self.runstate.clone();

        match newstate {
            RunState::InGame => {
                self.run_continuous_systems(ctx);

                let response_needed = self.player_input(ctx);
                if response_needed {
                    self.turn_counter += 1;
                    self.run_response_systems();
                }
            }
            RunState::MainMenu(menu_idx) => {
                newstate = run_menu_systems(self, ctx, menu_idx.0);
            }
        }

        self.runstate = newstate;
    }
}

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
        }
    };

    main_loop(context, gs)
}

/// Creates a new map and setups world for the start of a fresh run
pub fn start_new_game(world: &mut World, seed: u64) -> Map {
    let cb = named_monster_builder(&ENTITY_DB.lock().unwrap(), "Centipede", Position::new(5, 9));
    if let Some(mut cb) = cb {
        world.spawn(cb.build());
    }

    let (map, player_start) = generate_map(seed, 0);

    let player_builder = named_monster_builder(&ENTITY_DB.lock().unwrap(), "Player", player_start);
    if let Some(mut pb) = player_builder {
        world.spawn(pb.build());
    }

    map
}

#[derive(Deserialize)]
pub struct Config {
    fullscreen: bool,
    dev_mode: bool,
    font_file: String,
    font_size: usize,
    screensize_x: usize,
    screensize_y: usize,
    world_seed: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fullscreen: false,
            dev_mode: false,
            font_file: "Yayo.png".to_string(),
            font_size: 8,
            screensize_x: 120,
            screensize_y: 80,
            world_seed: 1,
        }
    }
}
