use specs::prelude::*;
use super::{Viewshed, Monster, Map, Position, WantsToMelee, RunState};
use bracket_lib::prelude::Point;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, WantsToMelee>);
    
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, player_entity, runstate, entities, mut viewshed, monster, mut position, mut wants_to_melee) = data;

        if *runstate != RunState::MonsterTurn { return; }
        for (entity, viewshed, _monster, pos) in (&entities, &mut viewshed, &monster, &mut position).join() {
            let monster_pos = Point::new(pos.x, pos.y);
            let current_distance = bracket_lib::geometry::DistanceAlg::Pythagoras.distance2d(monster_pos, *player_pos);
            if current_distance < 1.5 {
                // Attack here
                wants_to_melee.insert(entity, WantsToMelee { target: *player_entity }).expect("Unable to insert attack");
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                // Path to the player
                let path = bracket_lib::pathfinding::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32, // Monster position
                    map.xy_idx(player_pos.x, player_pos.y) as i32, // Player position
                    &mut *map
                );

                if path.success && path.steps.len() > 1 {
                    let mut idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = false;

                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    
                    idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = true;
                    viewshed.dirty = true;
                }
            }
        
        }
    }
}
