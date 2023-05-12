use crate::{
    actor::{change_floor, mine, player_attack, player_bump, MoveResult, Position},
    messagelog::Message,
    state::PlayerResponse,
    RunState, State,
};
use bracket_terminal::prelude::{BTerm, VirtualKeyCode};

/// Handles the action sent by the player
/// Returns the type of response needed based on what the player did
pub fn handle_player_action(state: &mut State, action: Action) -> PlayerResponse {
    let turn_sent = state.turn_counter;

    match action {
        Action::None => PlayerResponse::Waiting,
        Action::Wait => PlayerResponse::TurnAdvance,
        Action::Direction { delta } => match player_bump(&mut state.map, &mut state.world, delta.0) {
            MoveResult::Moved(_) => PlayerResponse::TurnAdvance,
            MoveResult::InvalidMove(msg) => {
                state.message_log.push(Message::new(msg, turn_sent));
                PlayerResponse::Waiting
            }
            MoveResult::Attack(target) => {
                player_attack(&mut state.world, &mut state.message_log, target, turn_sent);
                PlayerResponse::TurnAdvance
            }
            MoveResult::Mine(destructible) => {
                if mine(&mut state.map, &mut state.world, destructible, delta.0) {
                    PlayerResponse::TurnAdvance
                } else {
                    PlayerResponse::Waiting
                }
            }
        },
        Action::Ascend => match change_floor(&mut state.world, &mut state.map, -1) {
            true => PlayerResponse::StateChange(RunState::NextLevel(state.map.depth - 1)),
            false => PlayerResponse::Waiting,
        },
        Action::Descend => match change_floor(&mut state.world, &mut state.map, 1) {
            true => PlayerResponse::StateChange(RunState::NextLevel(state.map.depth + 1)),
            false => PlayerResponse::Waiting,
        },
        Action::SaveGame => PlayerResponse::StateChange(RunState::SaveGame),
    }
}

pub enum Action {
    None,
    Direction { delta: Position },
    Descend,
    Ascend,
    Wait,
    SaveGame, // this will probably change to a menu
}

pub fn player_input(ctx: &mut BTerm) -> Action {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::K => Action::Direction {
                delta: Position::new(0, -1),
            },
            VirtualKeyCode::Down | VirtualKeyCode::J => Action::Direction {
                delta: Position::new(0, 1),
            },
            VirtualKeyCode::Left | VirtualKeyCode::H => Action::Direction {
                delta: Position::new(-1, 0),
            },
            VirtualKeyCode::Right | VirtualKeyCode::L => Action::Direction {
                delta: Position::new(1, 0),
            },
            VirtualKeyCode::Y => Action::Direction {
                delta: Position::new(-1, -1),
            },
            VirtualKeyCode::U => Action::Direction {
                delta: Position::new(1, -1),
            },
            VirtualKeyCode::N => Action::Direction {
                delta: Position::new(-1, 1),
            },
            VirtualKeyCode::M => Action::Direction {
                delta: Position::new(1, 1),
            },
            VirtualKeyCode::Comma => Action::Ascend,
            VirtualKeyCode::Period => Action::Descend,
            VirtualKeyCode::Space => Action::Wait,
            VirtualKeyCode::Escape => Action::SaveGame,
            _ => Action::None,
        }
    } else {
        Action::None
    }
}
