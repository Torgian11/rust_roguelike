use bracket_lib::prelude::{ BTerm, RGB, RandomNumberGenerator };
use std::cmp::{max, min};

use super::rect::Rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}


pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];
    
    // Walls
    for x in 0..80 {
        map[xy_idx(x,0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Random splatting of walls
    let mut rng = RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1:i32, x2: i32, y: i32) {
    for x in min(x1, x2) ..= max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80*50 { // map size
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80*50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80*50];

    let mut rooms: Vec<Rect> = Vec::new();

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 3;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let room_width = rng.range(MIN_SIZE, MAX_SIZE);
        let room_height = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - room_width - 1) - 1;
        let y = rng.roll_dice(1, 50 - room_height - 1) - 1;

        let new_room = Rect::new(x, y, room_width, room_height);
        let mut ok = true;

        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len()-1].center();

                if rng.range(0,2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }
        }

        rooms.push(new_room);
        // rooms.push(Rect::new(x, y, room_width, room_height));
    }

    // apply_horizontal_tunnel(&mut map, 31, 35, 25);
    // apply_vertical_tunnel(&mut map, 5, 15, 21);

    (rooms, map)
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), bracket_lib::prelude::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), bracket_lib::prelude::to_cp437('#'));
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
