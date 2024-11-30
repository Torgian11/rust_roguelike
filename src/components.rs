use specs::prelude::*;
use specs_derive::*;
use bracket_lib::prelude::{Point, RGB};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub foreground: RGB,
    pub background: RGB,
    pub glyph: bracket_lib::prelude::FontCharType,
}

#[derive(Component, Debug)]
pub struct Monster {
    // pub name: Name
}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool
}
