use std::cmp;

use bracket_terminal::prelude::*;

use crate::{start_new_game, Config, RunState, State};

const MAINMENU_OPTIONS: [&'static str; 3] = ["New World", "Load Game", "Options"];

pub fn run_menu_systems(state: &mut State, ctx: &mut BTerm, menu_index: usize) -> RunState {
    let mut new_menu_index = menu_index;
    let mut error_message = Option::<String>::None;

    match ctx.key {
        Some(key) => match key {
            VirtualKeyCode::W => {
                new_menu_index = new_menu_index.saturating_sub(1);
            }
            VirtualKeyCode::S => {
                new_menu_index = cmp::min(new_menu_index + 1, MAINMENU_OPTIONS.len() - 1);
            }
            VirtualKeyCode::Return => {
                if menu_index == 0 {
                    state.map = start_new_game(&mut state.world, state.config.world_seed);
                    return RunState::InGame;
                } else if menu_index == 1 {
                    error_message = Some("No saved games found.".to_string());
                } else if menu_index == 2 {
                    error_message = Some("No options menu yet.".to_string());
                }
            }
            _ => (),
        },
        None => (),
    }
    draw_menu_screen(menu_index, ctx, error_message, &state.config);
    RunState::MainMenu(MenuIndex(new_menu_index))
}

pub fn draw_menu_screen(active_idx: usize, ctx: &mut BTerm, error_message: Option<String>, cfg: &Config) {
    let screen_size_x = cfg.screensize_x;
    let screen_size_y = cfg.screensize_y;

    ctx.draw_box(0, 0, screen_size_x - 1, screen_size_y - 1, WHITE, BLACK);
    ctx.print(20, 10, format!("Main Menu Index: {}", active_idx));
    ctx.print(8, screen_size_y / 2 - 5, "Terra Incognita");

    if let Some(err) = error_message {
        ctx.print(8, screen_size_y / 2 + 5, err);
    }

    for (idx, choice) in MAINMENU_OPTIONS.iter().enumerate() {
        if idx == active_idx {
            ctx.print_color(8, screen_size_y / 2 + idx, BLACK, WHITE, choice);
        } else {
            ctx.print_color(8, screen_size_y / 2 + idx, WHITE, BLACK, choice);
        }
    }
}

/// Holds a position to a selection in a menu
#[derive(Clone)]
pub struct MenuIndex(pub usize);
