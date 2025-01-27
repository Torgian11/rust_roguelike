use bracket_lib::prelude::{BTerm, Point, VirtualKeyCode};
use specs::prelude::*;

use super::{State, GameLog, Position, Item, Player, Map, Viewshed, RunState, CombatStats, WantsToMelee, WantsToPickupItem};
use std::cmp::{max, min};

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
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
            let mut player_pos = ecs.write_resource::<Point>();

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
            
            // Actions
            VirtualKeyCode::G => get_item(&mut game_state.ecs),
            VirtualKeyCode::I => return RunState::ShowInventory,
            VirtualKeyCode::D => return RunState::ShowDropItem,
            _ => { return RunState::AwaitingInput}
        },
    }
    RunState::PlayerTurn
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => gamelog.entries.push("There is nothing here to pick up!".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup.insert(*player_entity, WantsToPickupItem{collected_by: *player_entity, item}).expect("Unable to insert want to pickup");
        }
    }
}
