use bracket_lib::prelude::{BError, RGB};
use specs::prelude::*;

mod components;
pub use components::*;

mod rect;
pub use rect::Rect;

mod map;
pub use map::*;

mod player;
pub use player::*;
mod state;
use state::State;
mod visibility_system;
pub use visibility_system::VisibilitySystem;

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
    gs.ecs.register::<Viewshed>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    

    gs.ecs.create_entity()
    .with(Position { x: player_x, y: player_y})
    .with(Renderable {
        glyph: bracket_lib::prelude::to_cp437('&'),
        foreground: RGB::named(bracket_lib::color::YELLOW),
        background: RGB::named(bracket_lib::color::BLACK),
    })
    .with(Player{})
    .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
    .build();

    bracket_lib::prelude::main_loop(context, gs)
}
