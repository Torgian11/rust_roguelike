use bracket_lib::prelude::{RGB, RandomNumberGenerator};
use specs::prelude::*;
use super::{CombatStats, Player, Renderable, Name, Position, Viewshed, Monster, BlocksTile};

// Spawns player and returns their entity object.
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y})
        .with(Renderable {
            glyph: bracket_lib::prelude::to_cp437('@'),
            foreground: RGB::named(bracket_lib::color::YELLOW),
            background: RGB::named(bracket_lib::color::BLACK),
        })
        .with(Player{})
        .with(Name{name: "Player".to_string()})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5})
        .build()
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll :i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1,2);
    }
    match roll {
        1 => {orc(ecs, x, y)}
        _ => {goblin(ecs, x, y)}
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, bracket_lib::prelude::to_cp437('o'), "Orc");
}

fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, bracket_lib::prelude::to_cp437('g'), "Goblin");
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: bracket_lib::prelude::FontCharType, name: S) {
    ecs.create_entity()
        .with(Position {x, y})
        .with(Renderable {
            glyph,
            foreground: RGB::named(bracket_lib::color::RED),
            background: RGB::named(bracket_lib::color::BLACK),
        })
        .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
        .with(Monster{})
        .with(Name {name: name.to_string()})
        .with(BlocksTile{})
        .with(CombatStats{max_hp: 16, hp: 16, defense: 1, power: 4})
        .build();
        
}
