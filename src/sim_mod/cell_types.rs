use macroquad::color::*;
use crate::sim_mod::cell_types::CellType::{Leaf, Empty, Trunk};
use crate::traits::color_convert::ColorConvert;

#[derive(Copy, Clone)]
pub enum CellType {
    Empty,
    Leaf { sun_absorbed: f32},
    Trunk { sun_blocked: f32, root_connection: f32}
}

impl CellType {
    pub fn new_leaf() -> CellType {
        Leaf {
            sun_absorbed: 0.0,
        }
    }
    pub fn new_root() -> CellType {
        Trunk {
            sun_blocked: 0.,
            root_connection: 1.
        }
    }
}

impl ColorConvert for CellType {
    fn get_color(&self) -> Color {
        match self {
            Empty => {WHITE}
            Leaf { sun_absorbed } => {
                Color::new(
                    GREEN.r * sun_absorbed + BROWN.r * (1.-sun_absorbed),
                    GREEN.g * sun_absorbed + BROWN.g * (1.-sun_absorbed),
                    GREEN.b * sun_absorbed + BROWN.b * (1.-sun_absorbed),
                    1.)
            }
            Trunk { .. } => {BROWN}
        }
    }
}
