use bracket_lib::prelude::{BTerm, GameState};
use specs::prelude::*;

use super::draw_map;
use super::{VisibilitySystem, Position, Renderable, Map, MonsterAI};
use super::player_input;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }


pub struct State {
    pub ecs:World,
    pub runstate: RunState
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused
        } else {
            self.runstate = player_input(self, ctx);
        }
        
        
        draw_map(&self.ecs, ctx);
        

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();


        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
            }
        }
    }
}

