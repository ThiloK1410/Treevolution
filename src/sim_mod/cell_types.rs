use macroquad::color::*;
use crate::constants::simulation::LEAF_ABSORB_RATE;
use crate::sim_mod::cell_types::CellType::{Leaf, Empty, Trunk, Seed, Dead};
use crate::traits::color_convert::ColorConvert;


const DEAD_LEAF_COLOR: Color = Color {
    r: 0.729,
    g: 0.557,
    b: 0.137,
    a: 1.,
};
const DEAD_CELL_COLOR: Color = Color {
    r: 0.498,
    g: 0.439,
    b: 0.325,
    a: 1.,
};
#[derive(Copy, Clone)]
pub enum CellType {
    Empty,
    Leaf { sun_absorbed: f32},
    Trunk {root_connection: f32},
    Dead,
    Seed
}

impl CellType {
    pub fn new_leaf() -> CellType {
        Leaf {
            sun_absorbed: 1.,
        }
    }
    // creates a trunk which has full connection to root
    pub fn new_root() -> CellType {
        Trunk {
            root_connection: 1.
        }
    }

    pub fn get_root_con(&self) -> f32 {
        if let Trunk { root_connection, .. } = self {
            *root_connection
        } else {
            panic!("Tried to get root con from a non Trunk CellType");
        }
    }
}

impl ColorConvert for CellType {
    fn get_color(&self) -> Color {
        match self {
            Empty => {WHITE}
            Leaf { mut sun_absorbed } => {
                sun_absorbed /= LEAF_ABSORB_RATE;
                Color::new(
                    LIME.r * sun_absorbed + DEAD_LEAF_COLOR.r * (1.-sun_absorbed),
                    LIME.g * sun_absorbed + DEAD_LEAF_COLOR.g * (1.-sun_absorbed),
                    LIME.b * sun_absorbed + DEAD_LEAF_COLOR.b * (1.-sun_absorbed),
                    1.)
            }
            Trunk { .. } => {BROWN}
            Seed => {YELLOW}
            Dead => {GRAY}
        }
    }
}
