use macroquad::prelude::IVec2;
use crate::sim_mod::cell_types::CellType;
use crate::sim_mod::genome::Genome;
use crate::sim_mod::response::Response;

pub struct ResponseCluster {
    responses: [Response; 4],
}

impl ResponseCluster {
    pub fn new(genome: &mut Genome) -> ResponseCluster {
        let responses = [
            Response::new(genome),
            Response::new(genome),
            Response::new(genome),
            Response::new(genome)
        ];
        ResponseCluster {
            responses
        }
    }

    pub fn get_response_ix_with_position
        (&self, pos: &IVec2, current_energy: f32, root_connection: f32)
        -> Vec<(IVec2, usize, CellType)> {

        let mut out : Vec<(IVec2, usize, CellType)> = Vec::with_capacity(4);

        let (x, y) = (*pos).into();
        if self.responses[0].is_active(&pos, current_energy, root_connection) {
            out.push((
                IVec2::new(x, y+1),
                self.responses[0].get_cluster_index(),
                self.responses[0].get_cell_type()));
        }
        if self.responses[1].is_active(&pos, current_energy, root_connection) {
            out.push((
                IVec2::new(x+1, y),
                self.responses[1].get_cluster_index(),
                self.responses[1].get_cell_type()));
        }
        if self.responses[2].is_active(&pos, current_energy, root_connection) {
            out.push((
                IVec2::new(x, y-1),
                self.responses[2].get_cluster_index(),
                self.responses[2].get_cell_type()));
        }
        if self.responses[3].is_active(&pos, current_energy, root_connection) {
            out.push((
                IVec2::new(x-1, y),
                self.responses[3].get_cluster_index(),
                self.responses[3].get_cell_type()));
        }

        out
    }
    pub fn get_response(&self, direction: &str) -> &Response {
        match direction {
            "up" => &self.responses[0],
            "right" => &self.responses[1],
            "down" => &self.responses[2],
            "left" => &self.responses[3],
            _ => panic!("invalid direction"),
        }
    }
}