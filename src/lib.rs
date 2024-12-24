use crate::sim_mod::habitat::Habitat;
pub use crate::sim_mod::cell_types::CellType;
pub use crate::traits::color_convert::ColorConvert;

mod sim_mod;
mod constants;

mod traits;

pub fn create_habitat(grid_size: (i32, i32)) -> Habitat {
    Habitat::new(grid_size.into())
}