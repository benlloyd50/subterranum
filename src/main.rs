use bracket_terminal::prelude::*;
use hecs::*;

struct State {
    world: World,   // Holds all of our entities
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Bracket World");

        for (_, pos) in self.world.query::<With<&mut Position, &Player>>().iter() {
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::W => { pos.y -= 1;},
                    VirtualKeyCode::S => { pos.y += 1;},
                    VirtualKeyCode::A => { pos.x -= 1;},
                    VirtualKeyCode::D => { pos.x += 1;},
                    _ => {}
                }
            }

        }
        
        for (_, (pos,)) in self.world.query::<(&Position, )>().iter() {
            ctx.print(pos.x, pos.y, "☺");
        }
    }
}

struct Position {
    x: u32,
    y: u32,
}

struct Player;

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/Yayo.png");
fn main() {
    //Setup terminal renderer
    bracket_terminal::link_resource!(TILE_FONT, "resources/Yayo.png");
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .with_font("Yayo.png", 8, 8)
        .build()
        .unwrap();

    let mut world = World::new();

    let mover = world.spawn((Position{x:5, y:5}, Player));


    let gs: State = State {
        world,
    };
    main_loop(context, gs);
}

