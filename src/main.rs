use bracket_lib::prelude::{BError, RGB};
use specs::prelude::*;

mod components;
use components::*;

mod rect;

mod map;
use map::{TileType, new_map_rooms_and_corridors};

mod player;

mod state;
use state::State;

fn main() -> BError {
    use bracket_lib::prelude::BTermBuilder;

    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Gamestate
    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);

    let (player_x, player_y) = rooms[0].center();

    gs.ecs.create_entity()
    .with(Position { x: player_x, y: player_y})
    .with(Renderable {
        glyph: bracket_lib::prelude::to_cp437('&'),
        foreground: RGB::named(bracket_lib::color::YELLOW),
        background: RGB::named(bracket_lib::color::BLACK),
    })
    .with(Player{})
    .build();

    // for i in 0..10 {
    //     gs.ecs.create_entity()
    //     .with(Position { x: i * 7, y: 20})
    //     .with(Renderable {
    //         glyph: bracket_lib::prelude::to_cp437('☺'),
    //         foreground: RGB::named(bracket_lib::color::RED),
    //         background: RGB::named(bracket_lib::color::BLACK),
    //     })
    //     .build();
    // }

    bracket_lib::prelude::main_loop(context, gs)
}
