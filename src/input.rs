use crate::{
    actor::{try_ascend, try_descend, try_move, MoveResult, Name, Player, Position},
    combat::{attack, CombatStats},
    fov::ViewShed,
    monster::Breed,
    state::PlayerResponse,
    Message, RunState, State,
};
use std::cmp::max;

use bracket_terminal::prelude::{BTerm, VirtualKeyCode};
use hecs::With;

/// Handles the player input based on the key pressed
/// Returns the type of response needed based on what the player did
pub fn player_input(state: &mut State, ctx: &mut BTerm) -> PlayerResponse {
    let turn_sent = state.turn_counter;
    if let Some((e, ((pos, attacker_stats, name), view))) = &mut state
        .world
        .query::<(With<(&mut Position, &CombatStats, &Name), &Player>, &mut ViewShed)>()
        .iter()
        .next()
    {
        // dest_tile represents the position of something the player will interact with
        let mut dest_pos = pos.clone();
        if let Some(key) = ctx.key {
            // TODO: at some point I want to convert this into an action system where i can
            // queue up many actions or use input
            match key {
                VirtualKeyCode::Up | VirtualKeyCode::K => {
                    dest_pos.0.y = max(dest_pos.y() - 1, 0);
                }
                VirtualKeyCode::Down | VirtualKeyCode::J => {
                    dest_pos.0.y += 1;
                }
                VirtualKeyCode::Left | VirtualKeyCode::H => {
                    dest_pos.0.x = max(dest_pos.x() - 1, 0);
                }
                VirtualKeyCode::Right | VirtualKeyCode::L => {
                    dest_pos.0.x += 1;
                }
                VirtualKeyCode::Y => {
                    dest_pos.0.x = max(dest_pos.x() - 1, 0);
                    dest_pos.0.y = max(dest_pos.y() - 1, 0);
                }
                VirtualKeyCode::U => {
                    dest_pos.0.x += 1;
                    dest_pos.0.y = max(dest_pos.y() - 1, 0);
                }
                VirtualKeyCode::N => {
                    dest_pos.0.x = max(dest_pos.x() - 1, 0);
                    dest_pos.0.y += 1;
                }
                VirtualKeyCode::M => {
                    dest_pos.0.x += 1;
                    dest_pos.0.y += 1;
                }
                _ => {}
            }
            if !pos.0.eq(&dest_pos.0) {
                // if the dest_pos is not the same one they were standing on
                return match try_move(&mut state.map, &dest_pos, pos, view, *e) {
                    MoveResult::Acted(_) => PlayerResponse::TurnAdvance,
                    MoveResult::InvalidMove(reason) => {
                        state.message_log.push(Message::new(reason, turn_sent));
                        PlayerResponse::Waiting
                    }
                    MoveResult::Attack(target) => {
                        if let Ok(mut defender) = state.world.query_one::<(&mut CombatStats, &Breed)>(target) {
                            if let Some(defender) = defender.get() {
                                let damage_stmt =
                                    attack((defender.0, &defender.1.name), (attacker_stats, &name.0));
                                state.message_log.push(Message::new(damage_stmt, turn_sent));
                            }
                        } // Prevents stale enemies from being double despawned
                        PlayerResponse::TurnAdvance
                    }
                    MoveResult::Mine(_) => todo!("Gonna implement mining soon"),
                };
            }
            return match key {
                VirtualKeyCode::Comma => {
                    let depth = state.map.depth - 1;
                    if try_ascend(&state.map, pos, state.map.depth, 1) {
                        view.dirty = true;
                        return PlayerResponse::StateChange(RunState::NextLevel(depth));
                    }
                    PlayerResponse::Waiting
                }
                VirtualKeyCode::Period => {
                    let depth = state.map.depth + 1;
                    if try_descend(&state.map, pos) {
                        view.dirty = true;
                        return PlayerResponse::StateChange(RunState::NextLevel(depth));
                    }
                    PlayerResponse::Waiting
                }
                VirtualKeyCode::Space => {
                    // A waiting action
                    PlayerResponse::TurnAdvance
                }
                VirtualKeyCode::Escape => PlayerResponse::StateChange(RunState::SaveGame),
                _ => PlayerResponse::Waiting,
            };
        }
    }
    PlayerResponse::Waiting
}
