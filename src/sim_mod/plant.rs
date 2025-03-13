use crate::constants::simulation::{BASE_MAX_AGE, CELL_GROWTH_COST, DEFAULT_ENERGY, LIFETIME_FACTOR, MAX_AGE_CELL_MODIFIER, RESPONSE_SIZE, ROOT_CON_DECAY};
use crate::sim_mod::cell_types::CellType;
use crate::sim_mod::cell_types::CellType::Trunk;
use crate::sim_mod::genome::Genome;
use crate::sim_mod::plant_cell::PlantCell;
use crate::sim_mod::response_cluster::ResponseCluster;
use macroquad::math::IVec2;

pub struct Plant {
    cells: Vec<PlantCell>,
    pos: IVec2,
    has_root: bool,
    energy: f32,
    lifetime: usize,
    genome: Genome,
    response_clusters: Vec<ResponseCluster>,
}

impl Plant {
    pub fn new(pos: IVec2) -> Self {
        let mut genome = Genome::new();
        let responses = Self::create_responses(&mut genome);

        Self {
            cells: Vec::new(),
            pos,
            has_root: false,
            energy: DEFAULT_ENERGY,
            lifetime: 0,
            genome,
            response_clusters: responses,
        }
    }

    pub fn new_with_root(root_pos: IVec2) -> Self {
        let mut genome = Genome::new();
        let responses = Self::create_responses(&mut genome);

        Self {
            cells: vec![PlantCell::new_root(root_pos)],
            pos: root_pos,
            has_root: true,
            energy: DEFAULT_ENERGY,
            lifetime: 0,
            genome,
            response_clusters: responses,
        }
    }

    pub fn new_offspring(&self, pos: IVec2) -> Self {
        let mut genome = self.genome.create_offspring();
        let responses = Self::create_responses(&mut genome);

        Self {
            cells: vec![],
            pos,
            has_root: false,
            energy: DEFAULT_ENERGY + self.lifetime as f32 * LIFETIME_FACTOR,
            lifetime: 0,
            genome: self.genome.create_offspring(),
            response_clusters: responses,
        }
    }

    pub fn get_cells(&self) -> Vec<(IVec2, CellType)> {
        let mut out = Vec::<(IVec2, CellType)>::with_capacity(self.cells.len());
        for cell in &self.cells {
            out.push((cell.get_pos(), cell.get_cell_type()));
        }
        out
    }

    fn has_energy(&self) -> bool {
        self.energy > 0f32
    }

    pub fn get_energy(&self) -> f32 {
        self.energy
    }

    pub fn give_energy(&mut self, energy_amount: f32) {
        self.energy += energy_amount;
    }

    pub fn increase_age(&mut self) {
        self.lifetime += 1;
    }

    pub fn is_dead(&self) -> bool {
        self.is_too_old() || !self.has_root
    }

    fn is_too_old(&self) -> bool {
        self.lifetime > BASE_MAX_AGE + MAX_AGE_CELL_MODIFIER * self.cells.len()
    }
    pub fn create_growth_proposals(&self) -> Vec<(IVec2, usize, CellType, f32)> {
        // if plant has a root and can grow
        if self.has_root {
            let mut growth_proposals: Vec<(IVec2, usize, CellType, f32)> = Vec::new();
            for cell in self.cells.iter() {
                // only trunks can grow other trunks or leaves
                match cell.get_cell_type() {
                    // this is the parent for the growth proposals
                    Trunk { root_connection } => {
                        // we take the index of the response cluster
                        let idx = cell.get_response_ix();
                        // and use the index to get the corresponding cluster from the plant
                        let cluster = &self.response_clusters[idx];
                        // iterate over all active responses
                        for (pos, response_idx, mut cell_type) in cluster
                            .get_response_ix_with_position(
                                &cell.get_pos().into(),
                                self.energy,
                                root_connection,
                            )
                        {
                            // update root connection if cell_type is a trunk
                            match cell_type {
                                Trunk { root_connection } => {
                                    cell_type = Trunk {
                                        root_connection: root_connection * ROOT_CON_DECAY,
                                    };
                                }
                                _ => {}
                            }
                            // calculate the energy cost for growing
                            let energy_cost = CELL_GROWTH_COST / root_connection;
                            // add the responses as growth proposals
                            growth_proposals.push((pos, response_idx, cell_type, energy_cost));
                        }
                    }
                    _ => {}
                }
            }
            growth_proposals
        } else {
            // if plant is still a seed
            Vec::new()
        }
    }
    pub fn add_cell(&mut self, pos: IVec2, cell: CellType, response_ix: usize) {
        self.cells.push(PlantCell::new(pos, cell, response_ix));
    }

    // create an array of new response clusters with a size of RESPONSE_SIZE
    fn create_responses(genome: &mut Genome) -> Vec<ResponseCluster> {
        (0..RESPONSE_SIZE)
            .into_iter()
            .map(|x| ResponseCluster::new(genome))
            .collect()
    }

    pub fn is_seed(&self) -> bool {
        !self.has_root
    }

    pub fn get_pos(&self) -> IVec2 {
        self.pos
    }

    pub fn create_root(&mut self) {
        assert_eq!(self.pos.y, 0);
        self.has_root = true;
        self.cells.push(PlantCell::new_root(self.pos));
    }

    pub fn set_pos(&mut self, pos: IVec2) {
        assert!(!self.has_root);
        self.pos = pos;
    }
}
