use bracket_terminal::prelude::*;

use crate::State;

pub fn draw_gui(ctx: &mut BTerm, state: &State) {
    let screenheight = state.config.screensize_y;
    draw_message_box(ctx, state, screenheight);
    draw_right_box(ctx, state, screenheight);
}

fn draw_message_box(ctx: &mut BTerm, _state: &State, screenheight: usize) {
    ctx.draw_box(0, screenheight - 10, 99, 9, WHITE, BLACK);

    let lastmsgidx = _state.message_log.len() - 1;
    let mut msg_offset = 0;
    for i in lastmsgidx.saturating_sub(7)..=lastmsgidx {
        ctx.print(1, screenheight - 9 + msg_offset, &_state.message_log[i].contents);
        msg_offset += 1;
    }
}

fn draw_right_box(ctx: &mut BTerm, _state: &State, screenheight: usize) {
    let right_map_edge_x = 100;
    ctx.draw_box(right_map_edge_x, 0, 19, screenheight - 1, WHITE, BLACK);
    ctx.print(right_map_edge_x + 1, 1, "TerraIncognita");
    ctx.print(right_map_edge_x + 1, 2, "Dev Build V0.1.0");
}
