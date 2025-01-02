use bracket_lib::prelude::{RGB, Console, BTerm, Point};
use bracket_lib::color::{BLACK, GREY, MAGENTA, RED, WHITE};
use specs::prelude::*;
use super::{CombatStats, Player, GameLog, Map, Name, Position};

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let log = ecs.fetch::<GameLog>();

    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!("HP: {}/{}", stats.hp, stats.max_hp);
        ctx.print_color(1, 43, RGB::named(RED), RGB::named(BLACK), health);
        ctx.draw_bar_horizontal(10, 43, 30, stats.hp, stats.max_hp, RGB::named(RED), RGB::named(BLACK));
    }

    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 {ctx.print(2, y, s);}
        y+=1;
    }

    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, MAGENTA);

    fn draw_tooltips(ecs: &World, ctx: &mut BTerm) {
        let map = ecs.fetch::<Map>();
        let names = ecs.read_storage::<Name>();
        let positions = ecs.read_storage::<Position>();

        let mouse_pos = ctx.mouse_pos();
        if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { return; }
        let mut tooltip: Vec<String> = Vec::new();

        for (name, position) in (&names, &positions).join() {
            let idx = map.xy_idx(position.x, position.y);
            if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
                tooltip.push(name.name.to_string());
            }
        }

        if !tooltip.is_empty() {
            let mut width:i32 = 0;
            for ttip in tooltip.iter() {
                if width < ttip.len() as i32 { width = ttip.len() as i32; }
            }

            width += 3;

            if mouse_pos.0 > 40 {
                let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
                let left_x = mouse_pos.0 - width;
                let mut y = mouse_pos.1;

                for ttip in tooltip.iter() {
                    ctx.print_color(left_x, y, WHITE, GREY, ttip);
                    let padding = (width - ttip.len() as i32) - 1;
                    for i in 0..padding {
                        ctx.print_color(arrow_pos.x - i, y, WHITE, GREY, &" ".to_string());
                    }
                    y += 1;
                }

                ctx.print_color(arrow_pos.x, arrow_pos.y, WHITE, GREY, &"->".to_string());
            } else {
                let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
                let left_x = mouse_pos.0 + 3;
                let mut y = mouse_pos.1;

                for ttip in tooltip.iter() {
                    ctx.print_color(left_x + 1, y, WHITE, GREY, ttip);
                    let padding = (width - ttip.len() as i32) - 1;
                    for i in 0..padding {
                        ctx.print_color(arrow_pos.x + 1 + i, y, WHITE, GREY, &" ".to_string());
                    }
                    y += 1;
                }
                ctx.print_color(arrow_pos.x, arrow_pos.y, WHITE, GREY, &"<-".to_string());
            }
        }
    }
}
