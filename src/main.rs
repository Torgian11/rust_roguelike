use bracket_lib::prelude::{BError, RGB, RandomNumberGenerator, Point};
use specs::prelude::*;

mod components;
pub use components::*;

mod map;
pub use map::*;
mod monster_ai_system;
pub use monster_ai_system::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;

mod visibility_system;
pub use visibility_system::VisibilitySystem;
mod map_indexing_system;
pub use map_indexing_system::*;
mod damage_system;
pub use damage_system::*;
mod melee_combat_system;
mod gui;
pub use gui::*;
mod gamelog;
pub use gamelog::*;
mod spawner;
pub use spawner::*;
mod inventory_system;
pub use inventory_system::*;

mod state;
pub use state::{State, RunState};

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
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToDrinkPotion>();
    gs.ecs.register::<WantsToDropItem>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(RandomNumberGenerator::new());
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome to the Rust powered Roguelike".to_string()]});
    
    
    bracket_lib::prelude::main_loop(context, gs)
}
