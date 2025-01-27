use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
use bracket_lib::prelude::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Player>
                    );
    
    fn run (&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent,viewshed,pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y <= map.height );

                // If this is the player, then we can reveal what the player can see
                // TODO Extend this for monsters too?
                let _plyr : Option<&Player> = player.get(ent);
                if let Some(_plyr) = _plyr {
                    for vt in map.visible_tiles.iter_mut() {
                        *vt = false
                    };

                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
            

            // let plyr: Option<&Player> = player.get(ent);
            // if let Some(_plyr) = plyr {
            //     for vis in viewshed.visible_tiles.iter() {
            //         let idx = map.xy_idx(vis.x, vis.y);
            //         map.revealed_tiles[idx] = true;
                    
            //     }
            // }
        }
    }
}
