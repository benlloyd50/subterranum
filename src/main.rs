use std::process::exit;

use bracket_terminal::prelude::*;
use hecs::*;

mod map;
mod menu;
mod monster;
use map::{generate_map, render_map, Map, MAP_HEIGHT, MAP_WIDTH};
mod fov;
use fov::{update_vision, ViewShed};
mod actor;
mod tiles;
use actor::{render_entities, try_move, CharSprite, Player, Position};
use menu::run_menu_systems;
use monster::handle_monster_turns;

use crate::{
    menu::MenuIndex,
    monster::{Breed, MonsterType},
};

pub const SCREEN_SIZE_Y: usize = 80;
pub const SCREEN_SIZE_X: usize = 120;

pub struct State {
    world: World, // Holds all of our entities
    map: Map,     // Holds the tiles to the world
    // player_e: Entity // The player's entity for convienent lookup
    runstate: RunState,
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

        render_map(ctx, &self.map);
        render_entities(ctx, &self);
        ctx.print_color(0, 79, WHITESMOKE, BLACK, format!("FPS: {}", ctx.fps));
    }

    /// Checks for player's input and runs corresponding action
    /// Returns True if the player's action requires a response
    fn player_input(&mut self, ctx: &mut BTerm) -> bool {
        for (_, (pos, view)) in self
            .world
            .query::<(With<&mut Position, &Player>, &mut ViewShed)>()
            .iter()
        {
            // dest_tile represents the position of something the player will interact with
            let mut dest_pos = pos.clone();
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::W => {
                        dest_pos.y = dest_pos.y.saturating_sub(1);
                        return try_move(&self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::S => {
                        dest_pos.y += 1;
                        return try_move(&self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::A => {
                        dest_pos.x = dest_pos.x.saturating_sub(1);
                        return try_move(&self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::D => {
                        dest_pos.x += 1;
                        return try_move(&self.map, dest_pos, pos, view);
                    }
                    VirtualKeyCode::Escape => exit(0),
                    _ => {}
                }
            }
        }
        false
    }

    /// Response systems are ran after a player inputs something that progresses a turn
    fn run_response_systems(&mut self) {
        handle_monster_turns(self);
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

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/Yayo.png");

fn main() -> BError {
    //Setup terminal renderer
    bracket_terminal::link_resource!(TILE_FONT, "resources/Yayo.png");
    let context = BTermBuilder::simple(SCREEN_SIZE_X, SCREEN_SIZE_Y)?
        .with_title("Terra Incognita [ALPHA]")
        .with_font("Yayo.png", 8, 8)
        .with_fullscreen(false) // this could be toggled with a config file! in the future...
        .build()?;

    // Initialize the bare minimum to get into the main menu for the rest of the game initialization
    let mut world = World::new();

    let dev_mode = true;
    let gs = if dev_mode {
        // For dev purposes, we can skip the main menu
        let map = start_new_game(&mut world);
        State {
            world,
            map,
            runstate: RunState::InGame,
        }
    } else {
        State {
            world,
            map: Map::empty(),
            runstate: RunState::MainMenu(MenuIndex(0)),
        }
    };

    main_loop(context, gs)
}

/// Creates a new map and setups world for the start of a fresh run
pub fn start_new_game(world: &mut World) -> Map {
    world.spawn((
        Position::new(5, 5),
        CharSprite::new('☺', CYAN, None),
        Player,
        ViewShed::new(8),
    ));

    world.spawn((
        Breed::new(MonsterType::Centipede),
        CharSprite::new('c', ROSY_BROWN, None),
        Position::new(9, 10),
        ViewShed::new(3),
    ));

    world.spawn((Position::new(10, 12), CharSprite::new('@', YELLOW, None)));

    generate_map()
}
