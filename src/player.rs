use bracket_lib::prelude::{BTerm, Point, VirtualKeyCode};
use bracket_lib::terminal::console;
use specs::prelude::*;

use super::{State, TileType, Position, Player, Map, Viewshed, RunState, CombatStats, WantsToMelee};
use std::cmp::{max, min};

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut player_pos = ecs.write_resource::<Point>();

    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
        if pos.x + delta_x < 1 || pos.x + delta_x > map.width - 1 || pos.y + delta_y < 1 || pos.y + delta_y > map.height - 1 { return; }

        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);

            if let Some(_target) = target {
                wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("Add target failed");
                return;
            }
        }
        if !map.blocked[destination_idx] {
            pos.x = min(79, max (0, pos.x + delta_x));
            pos.y = min(79, max (0, pos.y + delta_y));

            viewshed.dirty = true;

            player_pos.x = pos.x;
            player_pos.y = pos.y;
        }
    }
}

pub fn player_input(game_state: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut game_state.ecs),
            VirtualKeyCode::H |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut game_state.ecs),
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L |
            VirtualKeyCode::Right => try_move_player(1, 0, &mut game_state.ecs),
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K |
            VirtualKeyCode::Up => try_move_player(0, -1, &mut game_state.ecs),

            // Diaganols
            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::Y => try_move_player(1, -1, &mut game_state.ecs),
            VirtualKeyCode::Numpad7 | 
            VirtualKeyCode::U => try_move_player(-1, -1, &mut game_state.ecs),
            VirtualKeyCode::Numpad1 | 
            VirtualKeyCode::B => try_move_player(-1, 1, &mut game_state.ecs),
            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::N => try_move_player(1, 1, &mut game_state.ecs),
            
            _ => { return RunState::AwaitingInput}
        },
    }
    RunState::PlayerTurn
}
