use specs::prelude::*;
use super::{Viewshed, Name, Monster};
use bracket_lib::prelude::{Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>);
    
    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, monster, name) = data;
        for (viewshed, monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", &name.name));
            }
            
        }
    }
}
