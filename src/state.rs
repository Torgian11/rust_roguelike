use bracket_lib::prelude::{BTerm, GameState};
use specs::prelude::*;

use super::map::{TileType, draw_map};
use super::components::*;
use super::player::player_input;

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();


        player_input(self, ctx);
        self.run_systems();
        
        
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    }
}
