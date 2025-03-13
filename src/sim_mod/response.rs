use macroquad::math::IVec2;
use crate::sim_mod::cell_types::CellType;
use crate::constants::simulation::{CELL_GROWTH_COST, GRID_SIZE, HEIGHT_THRESHOLD_CHANCE, RESPONSE_SIZE};
use crate::sim_mod::genome::Genome;

pub struct Response {
    target_cluster_index: usize,
    height_threshold: i32,
    growth_bias_factor: f32,
    target_cell_type: CellType,
}

impl Response {
    pub fn new(genome: &mut Genome) -> Response {
        Response {
            target_cluster_index: Self::parse_cluster_index(genome.parse_value()),
            height_threshold: {
                let value = genome.parse_value_normalized();
                match genome.parse_value_normalized() {
                    n if n <= HEIGHT_THRESHOLD_CHANCE => (value * GRID_SIZE.y as f32) as i32,
                    _ => 0
                }
            },
            growth_bias_factor: 1. / genome.parse_value_normalized(),
            target_cell_type: {
                match genome.parse_bool() {
                    true => {
                        CellType::new_leaf()
                    }
                    false => {
                        CellType::new_root()
                    }
                }
            }
        }
    }
    pub fn get_cluster_index(&self) -> usize {
        self.target_cluster_index
    }

    pub fn is_active(&self, pos: &IVec2, current_energy: f32, root_connection: f32) -> bool {
        let height_reached = self.height_threshold == 0 || pos.y >= self.height_threshold;
        let enough_energy = current_energy >= CELL_GROWTH_COST
            / self.growth_bias_factor
            / root_connection;

        height_reached && enough_energy
    }

    pub fn get_cell_type(&self) -> CellType {
        self.target_cell_type
    }

    //gives a random number 0<=x<Response_SIZE to act as a random index for all response clusters of a plant
    fn parse_cluster_index(x: u16) -> usize {
        x as usize % RESPONSE_SIZE
    }

}