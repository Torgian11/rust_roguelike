use bracket_lib::prelude::{BError, RGB, RandomNumberGenerator, Point};
use specs::prelude::*;

mod components;
pub use components::*;

mod rect;
pub use rect::Rect;

mod map;
pub use map::*;
mod monster_ai_system;
pub use monster_ai_system::*;
mod player;
pub use player::*;
mod state;
pub use state::{State, RunState};
mod visibility_system;
pub use visibility_system::VisibilitySystem;
mod map_indexing_system;
pub use map_indexing_system::*;
mod damage_system;
pub use damage_system::*;
mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;
mod gui;
pub use gui::*;
mod gamelog;
pub use gamelog::*;

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

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (monster_x, monster_y) = room.center();
        let glyph: bracket_lib::prelude::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {glyph = bracket_lib::prelude::to_cp437('g'); name = "Goblin".to_string();}
            _ => {glyph = bracket_lib::prelude::to_cp437('o'); name = "Orc".to_string();}
        }

        gs.ecs.create_entity()
            .with(Position {x: monster_x, y: monster_y})
            .with(Renderable {
                glyph: glyph,
                foreground: RGB::named(bracket_lib::color::RED),
                background: RGB::named(bracket_lib::prelude::BLACK),
            })
            .with(Monster{})
            .with(Name{name: format!("{} {}", &name, i)})
            .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
            .with(BlocksTile{})
            .with(CombatStats{max_hp: 16, hp: 16, defense: 1, power: 4})
            .build();
    }
    
    
    let player_entity = gs.ecs.create_entity()
        .with(Position { x: player_x, y: player_y})
        .with(Renderable {
            glyph: bracket_lib::prelude::to_cp437('&'),
            foreground: RGB::named(bracket_lib::color::YELLOW),
            background: RGB::named(bracket_lib::color::BLACK),
        })
        .with(Player{})
        .with(Name{name: "Player".to_string()})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5})
        .build();
    
    gs.ecs.insert(player_entity);
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome to the Rust powered Roguelike".to_string()]});
    gs.ecs.insert(RunState::PreRun);
    
    bracket_lib::prelude::main_loop(context, gs)
}
