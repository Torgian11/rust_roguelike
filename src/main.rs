use bracket_lib::prelude::{BError, RGB};
use specs::prelude::*;
use roguelike::{LeftMover, Player, Position, Renderable, State};

use tile_system;

fn main() -> BError {
    use bracket_lib::prelude::BTermBuilder;

    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Gamestate
    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.insert(tile_system::new_map());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs.create_entity()
    .with(Position { x: 40, y: 25})
    .with(Renderable {
        glyph: bracket_lib::prelude::to_cp437('&'),
        foreground: RGB::named(bracket_lib::color::YELLOW),
        background: RGB::named(bracket_lib::color::BLACK),
    })
    .with(Player{})
    .build();

    for i in 0..10 {
        gs.ecs.create_entity()
        .with(Position { x: i * 7, y: 20})
        .with(Renderable {
            glyph: bracket_lib::prelude::to_cp437('☺'),
            foreground: RGB::named(bracket_lib::color::RED),
            background: RGB::named(bracket_lib::color::BLACK),
        })
        .with(LeftMover{})
        .build();
    }

    bracket_lib::prelude::main_loop(context, gs)
}
