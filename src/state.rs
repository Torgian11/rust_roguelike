use bracket_lib::prelude::{BTerm, GameState};
use specs::prelude::*;

use super::draw_map;
use super::{VisibilitySystem, Position, Renderable};
use super::player_input;


pub struct State {
    pub ecs:World
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();


        player_input(self, ctx);
        self.run_systems();
        
        draw_map(&self.ecs, ctx);
        

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    }
}

