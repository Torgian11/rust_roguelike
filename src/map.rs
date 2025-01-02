use bracket_lib::prelude::{ Algorithm2D, BTerm, BaseMap, Point, RandomNumberGenerator, RGB, SmallVec };
use std::cmp::{max, min};
use super::Rect;
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

const MAPWIDTH: usize = 80;
const MAPHEIGHT: usize = 50;
// Map sizing
const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        bracket_lib::geometry::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx:usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x-1, y) {exits.push((idx-1, 1.0))};
        if self.is_exit_valid(x+1, y) {exits.push((idx+1, 1.0))};
        if self.is_exit_valid(x, y-1) {exits.push((idx-w, 1.0))};
        if self.is_exit_valid(x, y+1) {exits.push((idx+w, 1.0))};

        // Diaganols
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w) - 1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w) + 1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w) - 1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w) + 1, 1.45)); }

        exits
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

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
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
            tiles: vec![TileType::Wall; MAPCOUNT],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            tile_content: vec![Vec::new(); MAPCOUNT]
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
        let mut map = vec![TileType::Floor; MAPCOUNT];
        
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

    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }
    // fn is_tile_blocked(&self, x:i32, y:i32) -> bool {
    //     // if x/y tile is blocked by player or monster, return true
    //     let tile_idx = self.xy_idx(x, y);

    //     if self.tiles[idx as usize] ==  {
    //         true
    //     }

    //     false
    // }
    
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
