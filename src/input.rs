use crate::{
    actor::{try_ascend, try_descend, try_move, Player, Position},
    fov::ViewShed,
    state::PlayerResponse,
    RunState, State,
};
use std::{cmp::max, process::exit};

use bracket_terminal::prelude::{BTerm, VirtualKeyCode};
use hecs::With;

/// Handles the player input based on the key pressed
/// Returns the type of response needed based on what the player did
pub fn player_input(state: &mut State, ctx: &mut BTerm) -> PlayerResponse {
    let mut should_respond = false;
    for (e, (pos, view)) in state
        .world
        .query::<(With<&mut Position, &Player>, &mut ViewShed)>()
        .iter()
    {
        // dest_tile represents the position of something the player will interact with
        let mut dest_pos = pos.clone();
        if let Some(key) = ctx.key {
            // TODO: at some point I want to convert this into an action system where i can
            // queue up many actions or use input
            match key {
                VirtualKeyCode::Up | VirtualKeyCode::K => {
                    dest_pos.0.y = max(dest_pos.y() - 1, 0);
                    should_respond = try_move(&mut state.map, &dest_pos, pos, view, Some(e));
                }
                VirtualKeyCode::Down | VirtualKeyCode::J => {
                    dest_pos.0.y += 1;
                    should_respond = try_move(&mut state.map, &dest_pos, pos, view, Some(e));
                }
                VirtualKeyCode::Left | VirtualKeyCode::H => {
                    dest_pos.0.x = max(dest_pos.x() - 1, 0);
                    should_respond = try_move(&mut state.map, &dest_pos, pos, view, Some(e));
                }
                VirtualKeyCode::Right | VirtualKeyCode::L => {
                    dest_pos.0.x += 1;
                    should_respond = try_move(&mut state.map, &dest_pos, pos, view, Some(e));
                }
                VirtualKeyCode::Y => {
                    dest_pos.0.x = max(dest_pos.x() - 1, 0);
                    dest_pos.0.y = max(dest_pos.y() - 1, 0);
                    should_respond = try_move(&mut state.map, &dest_pos, pos, view, Some(e));
                }
                VirtualKeyCode::U => {
                    dest_pos.0.x += 1;
                    dest_pos.0.y = max(dest_pos.y() - 1, 0);
                    should_respond = try_move(&mut state.map, &dest_pos, pos, view, Some(e));
                }
                VirtualKeyCode::N => {
                    dest_pos.0.x = max(dest_pos.x() - 1, 0);
                    dest_pos.0.y += 1;
                    should_respond = try_move(&mut state.map, &dest_pos, pos, view, Some(e));
                }
                VirtualKeyCode::M => {
                    dest_pos.0.x += 1;
                    dest_pos.0.y += 1;
                    should_respond = try_move(&mut state.map, &dest_pos, pos, view, Some(e));
                }
                VirtualKeyCode::Comma => {
                    let depth = state.map.depth - 1;
                    if try_ascend(&state.map, pos, state.map.depth, 1) {
                        return PlayerResponse::StateChange(RunState::NextLevel(depth));
                    }
                }
                VirtualKeyCode::Period => {
                    let depth = state.map.depth + 1;
                    if try_descend(&state.map, pos) {
                        return PlayerResponse::StateChange(RunState::NextLevel(depth));
                    }
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
    if should_respond {
        PlayerResponse::TurnAdvance
    } else {
        PlayerResponse::Waiting
    }
}
