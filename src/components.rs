use specs::prelude::*;
use specs_derive::*;
use bracket_lib::prelude::RGB;

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
pub struct Player {}

