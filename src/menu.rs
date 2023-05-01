use std::{cmp, fs};

use bracket_terminal::prelude::*;

use crate::{save_system::start_load_game, start_new_game, Config, RunState, State};

const MAINMENU_OPTIONS: [&str; 3] = ["New World", "Load Game", "Options"];

pub fn run_menu_systems(state: &mut State, ctx: &mut BTerm, menu_index: usize) -> RunState {
    let mut new_menu_index = menu_index;
    let mut error_message = Option::<String>::None;

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::K | VirtualKeyCode::Up => {
                new_menu_index = new_menu_index.saturating_sub(1);
            }
            VirtualKeyCode::J | VirtualKeyCode::Down => {
                new_menu_index = cmp::min(new_menu_index + 1, MAINMENU_OPTIONS.len() - 1);
            }
            VirtualKeyCode::Return => {
                if menu_index == 0 {
                    state.map = start_new_game(&mut state.world, state.config.world_seed);
                    return RunState::InGame;
                } else if menu_index == 1 {
                    match fs::File::open("./saves/player.sav") {
                        Ok(..) => {
                            *state = start_load_game(state.config.clone());
                            return RunState::InGame;
                        }
                        Err(..) => {
                            error_message = Some("No saved games found.".to_string());
                        }
                    }
                } else if menu_index == 2 {
                    error_message = Some("No options menu yet.".to_string());
                }
            }
            _ => (),
        }
        ctx.cls();
    }

    draw_menu_screen(new_menu_index, ctx, error_message, &state.config);
    RunState::MainMenu(MenuIndex(new_menu_index))
}

pub fn draw_menu_screen(active_idx: usize, ctx: &mut BTerm, error_message: Option<String>, cfg: &Config) {
    let screen_size_x = cfg.screensize_x;
    let screen_size_y = cfg.screensize_y;

    if cfg.dev_mode {
        // Debug information
        ctx.print(20, 10, format!("Main Menu Index: {}", active_idx));
    }

    if let Some(err) = error_message {
        ctx.print(10, screen_size_y / 2 + 9, err);
    }

    let xp_file = XpFile::from_resource("../resources/rex/intro_screen.xp").unwrap();
    ctx.render_xp_sprite(&xp_file, 2, 17);
    ctx.print(4, 37, "Developed By: Benjamin Lloyd");
    ctx.draw_hollow_box(0, 0, screen_size_x - 1, screen_size_y - 1, WHITE, BLACK);

    draw_option_box(ctx, screen_size_y, active_idx);
}

/// Draws a rex paint box and adds text to display options for the player to pick
fn draw_option_box(ctx: &mut BTerm, screen_size_y: usize, active_idx: usize) {
    let options_anchor = screen_size_y / 2 + 3;
    let xp_file = XpFile::from_resource("../resources/rex/options_box.xp").unwrap();
    ctx.render_xp_sprite(&xp_file, 10, options_anchor as i32 - 2);

    for (idx, choice) in MAINMENU_OPTIONS.iter().enumerate() {
        if idx == active_idx {
            ctx.print_color(18, options_anchor + idx, BLACK, WHITE, choice);
            ctx.set(17, options_anchor + idx, LIGHT_BLUE, BLACK, to_cp437('►'));
            ctx.set(27, options_anchor + idx, LIGHT_BLUE, BLACK, to_cp437('◄'));
        } else {
            ctx.print_color(18, options_anchor + idx, WHITE, BLACK, choice);
        }
    }
}

/// Holds a position to a selection in a menu
#[derive(Clone)]
pub struct MenuIndex(pub usize);
