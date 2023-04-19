use std::collections::HashMap;

use bracket_terminal::prelude::{BTerm, GameState};
use hecs::World;

use crate::{
    actor::render_entities, fov::update_vision, gui::draw_gui, input::player_input, map::{render_map, Map},
    menu::{run_menu_systems, MenuIndex}, messagelog::Message, monster::handle_monster_turns, worldgen::move_to_new_floor, config::Config,
};

pub struct State {
    pub world: World, // Holds all of our entities
    pub map: Map,     // Holds the tiles to the world
    // player_e: Entity // The player's entity for convienent lookup
    pub runstate: RunState,
    pub message_log: Vec<Message>,
    pub config: Config,
    pub turn_counter: usize,
    pub generated_maps: HashMap<usize, Map>,
}

#[derive(Clone)]
pub enum RunState {
    InGame,
    MainMenu(MenuIndex),
    NextLevel(usize),
}

pub enum PlayerResponse {
    StateChange(RunState),
    TurnAdvance,
    Waiting,
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

                match player_input(self, ctx) {
                    PlayerResponse::StateChange(new_state) => newstate = new_state,
                    PlayerResponse::TurnAdvance => {
                        self.turn_counter += 1;
                        self.run_response_systems();
                    }
                    _ => {}
                }
            }
            RunState::MainMenu(menu_idx) => {
                newstate = run_menu_systems(self, ctx, menu_idx.0);
            }
            RunState::NextLevel(new_depth) => {
                move_to_new_floor(self, new_depth);
                newstate = RunState::InGame;
            }
        }

        self.runstate = newstate;
    }
}
