use bracket_lib::prelude::{ Algorithm2D, BTerm, BaseMap, Point, RandomNumberGenerator, RGB };
use std::cmp::{max, min};
use super::Rect;
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_horizontal_tunnel(&mut self, x1:i32, x2: i32, y: i32) {
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize { // map size
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    } 
    
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80*50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80*50],
            visible_tiles: vec![false; 80*50]
        };
        
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
    
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);
    
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
    
                    if rng.range(0,2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }
            }
    
            map.rooms.push(new_room);
        }

        map
    }

    /// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
    /// look awful.
    pub fn new_map_test(&self) -> Vec<TileType> {
        let mut map = vec![TileType::Floor; 80*50];
        
        // Walls
        for x in 0..80 {
            map[self.xy_idx(x,0)] = TileType::Wall;
            map[self.xy_idx(x, 49)] = TileType::Wall;
        }
        for y in 0..50 {
            map[self.xy_idx(0, y)] = TileType::Wall;
            map[self.xy_idx(79, y)] = TileType::Wall;
        }

        // Random splatting of walls
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = self.xy_idx(x, y);
            if idx != self.xy_idx(40, 25) {
                map[idx] = TileType::Wall;
            }
        }

        map
    }

        
       
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();
    let mut x = 0;
    let mut y = 0;
    
    for (idx, tile) in map.tiles.iter().enumerate() {
        // Render tile based on tile type
        if map.revealed_tiles[idx] {
            let glyph;
            let mut foreground;
            match tile {
                TileType::Floor => {
                    glyph = bracket_lib::prelude::to_cp437('.');
                    foreground = RGB::from_f32(0.0, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = bracket_lib::prelude::to_cp437('#');
                    foreground = RGB::from_f32(0., 1.0, 0.);
                }
            }
            if !map.visible_tiles[idx] { foreground = foreground.to_greyscale() }
            ctx.set(x,y,foreground,RGB::from_f32(0., 0., 0.), glyph);
        }
        
        // Move coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
