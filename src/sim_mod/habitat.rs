use std::collections::HashMap;
use crate::constants::simulation::{CELL_SUSTAIN_ENERGY_COST, DEAD_CELL_REMOVE_RATE, LEAF_ABSORB_RATE, MAX_GROWTHS_PER_ITERATION, SEED_ENERGY_DRAIN, SEED_SPAWN_RATE, SUN_POWER, TRUNK_ABSORB_RATE};
use crate::sim_mod::cell_types::CellType;
use crate::sim_mod::cell_types::CellType::{Empty, Leaf, Trunk, Dead};
use crate::sim_mod::plant::Plant;
use crate::traits::color_convert::ColorConvert;
use macroquad::math::IVec2;
use rand::{random_bool, Rng};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use macroquad::rand::{gen_range};

// control struct, to hold the data of every tree and information of whole grid
pub struct Habitat {
    grid_size: IVec2,
    cell_map: Vec<Vec<CellType>>,
    plants: Vec<Plant>,
    seeds: Vec<Plant>,
    dead_cells: Vec<IVec2>,
    ground_buffer: Vec<Vec<Plant>>,
    minimum_plants: usize,
    selected_pos: Option<IVec2>,
    selected_plant_ix: Option<usize>,
    selected_cell_ix: Option<usize>,
}

impl Habitat {
    pub fn new(grid_size: IVec2) -> Self {
        Self {
            grid_size,
            cell_map: vec![vec![Empty; grid_size.y as usize]; grid_size.x as usize],
            plants: Vec::new(),
            seeds: Vec::new(),
            dead_cells: Vec::new(),
            ground_buffer: {
                // cant use macro here because plant is not clone
                let mut out = vec![];
                out.resize_with(grid_size.x as usize, || Vec::new());
                out
            },
            minimum_plants: 0,
            selected_pos: None,
            selected_plant_ix: None,
            selected_cell_ix: None,
        }
    }

    // get the data of the grid as a linear rgb byte vector, every 3 bytes represent one pixel (rgb)
    // the data is ordered column by column
    pub fn get_rgb_data(&self) -> Vec<u8> {
        self.cell_map
            .iter()
            .flatten()
            .map(|x| x.get_color())
            .flat_map(|x| [x.r, x.g, x.b])
            .map(|x| (x * 255.0) as u8)
            .collect::<Vec<u8>>()
    }

    // main update loop, meant to be called in a loop
    pub fn update(&mut self) {

        // check if below minimum plants, if so spawn single plant
        if self.get_total_plant_count() < self.minimum_plants {
            self.spawn_plant()
        }

        // increase the age of all plants
        self.plants.par_iter_mut().for_each(|plant| plant.increase_age());

        // let each plant grow by the selected growths
        let plant_growth = self.create_growths();
        for (idx, growths) in plant_growth.into_iter().enumerate() {
            let mut plant = &mut self.plants[idx];
            for (pos,response_ix, cell_type, energy_cost) in growths {
                plant.add_cell(pos, cell_type, response_ix);
                plant.give_energy(-energy_cost);
            }
        }

        // collect all indices of dead plants
        let mut dead_plant_ix = Vec::<usize>::new();
        for (ix, plant) in self.plants.iter().enumerate() {
            if plant.is_dead() {
                dead_plant_ix.push(ix);
                //
                for cell_ix in 0..plant.get_cells().len() {
                    if random_bool(SEED_SPAWN_RATE as f64) {
                        let (pos, _) = plant.get_cells()[cell_ix];
                        self.seeds.push(plant.new_offspring(pos));
                    }
                }
            }
        }

        // collect all cell positions of dead plants
        for ix in dead_plant_ix {
            for (pos, _) in self.plants[ix].get_cells().iter() {
                self.dead_cells.push(*pos);
            }
        }

        // destroy all plants with no energy or which are too old
        self.plants.retain(|plant| !plant.is_dead());

        // add all plant_cells to fresh habitat grid
        self.apply_plants();

        // calculate the sun energy for each cell in the habitat grid
        self.update_all_columns();

        // look up the collected energy for each plant and add it while removing consumed energy
        self.supply_plant_energy();

        // let dead cells "deteriorate"
        self.dead_cells.retain(|_| !random_bool(DEAD_CELL_REMOVE_RATE as f64));

        // update the selected plant and cell indices
        if let Some(selected_pos) = self.selected_pos {
            match self.get_cell_at(selected_pos) {
                Leaf {..} | Trunk {..} => {
                    for (plant_ix, plant) in self.plants.iter().enumerate() {
                        if let Some(cell_ix) = plant.get_cell_ix_at(selected_pos) {
                            self.selected_plant_ix = Some(plant_ix);
                            self.selected_cell_ix = Some(cell_ix);
                        }
                    }
                }
                _ => {
                    // if there is no selectable cell at the selected position, set all selections to null
                    self.selected_plant_ix = None;
                    self.selected_cell_ix = None;
                    self.selected_pos = None
                }
            }
        }

        // --- right here calculations are done,
        // and it is allowed to change the grid for appearance(show seeds)

        // let seeds in the air drop, and if they touch the ground they will become grounded seeds
        self.update_seeds();

        self.update_grounded_buffer();

        // adding all seeds to grid, happens after calculation because seeds cant impact anything
        self.show_seeds();
    }

    // spawns a random seed in the grid
    pub fn spawn_plant(&mut self) {
        let mut rng = rand::rng();
        let pos = (rng.random_range(0..self.grid_size.x), self.grid_size.y-1);
        if let Empty = self.get_cell_at(pos.into()) {
            self.seeds.push(Plant::new(pos.into()));
        }
    }

    pub fn set_minimum_plants(&mut self, minimum_plants: usize) {
        self.minimum_plants = minimum_plants;
    }

    // returns the total amount of plants currently existing in the habitat,
    // including seeds, in the air or ground, and living plants
    pub fn get_total_plant_count(&self) -> usize {
        self.plants.len() + self.seeds.len()
            + self.ground_buffer.iter().map(|x| x.len()).sum::<usize>()
    }

    pub fn get_focus_information(&self) -> Option<HashMap<String, String>> {
        if let None = self.selected_pos {
            return None;
        }
        let mut information: HashMap<String, String> = HashMap::new();
        if let Some(selected_plant_ix) = self.selected_plant_ix {
            let plant = &self.plants[selected_plant_ix];
            information.insert("energy".into(), plant.get_energy().to_string());
            information.insert("cell_count".into(), plant.get_cells().len().to_string());
        }
        Some(information)
    }

    // selecting a pos outside of grid may panic
    pub fn select_pos(&mut self, pos: IVec2) {
        self.selected_pos = Some(pos);
    }

    fn apply_plants(&mut self) {
        // reinitialize whole grid as empty
        self.cell_map = vec![vec![Empty; self.grid_size.y as usize]; self.grid_size.x as usize];

        // collecting leaves and trunks separately because leaves have priority
        let mut leaves = Vec::<(IVec2, CellType)>::new();
        let mut trunks = Vec::<(IVec2, CellType)>::new();

        for plant in &self.plants {
            for (pos, cell) in plant.get_cells().iter() {
                match cell {
                    Leaf { .. } => {leaves.push((*pos, *cell));},
                    Trunk { .. } => {trunks.push((*pos, *cell));},
                    _ => {}
                }
            }
        }
        for (pos, cell) in trunks.iter() {
            self.set_cell(*pos, *cell)
        }
        for (pos, cell) in leaves.iter() {
            self.set_cell(*pos, *cell)
        }

        for ix in 0..self.dead_cells.len() {
            self.set_cell(self.dead_cells[ix], Dead);
        }
    }


    fn set_cell(&mut self, pos: IVec2, cell_type: CellType) {
        self.cell_map[((pos.x + self.grid_size.x) % self.grid_size.x) as usize]
            [(self.grid_size.y - 1 - pos.y) as usize] = cell_type;
    }



    fn is_in_grid(&self, pos: IVec2) -> bool {
        !(pos.x < 0 || pos.x >= self.grid_size.x || pos.y < 0 || pos.y >= self.grid_size.y)
    }
    fn is_in_y_bounds(&self, pos: IVec2) -> bool {
        !(pos.y < 0 || pos.y >= self.grid_size.y)
    }

    fn get_cell_at(&self, pos: IVec2) -> &CellType {
        &self.cell_map
            [((pos.x + self.grid_size.x) % self.grid_size.x) as usize]
            [(self.grid_size.y - 1 - pos.y) as usize]
    }



    fn spawn_random_cell(&mut self) {
        let mut rng = rand::rng();
        // try to place a cell at random position, 100 tries at max
        for _ in 0..100 {
            let pos = IVec2::new(
                rng.random_range(0..self.grid_size.x),
                rng.random_range(0..self.grid_size.y),
            );
            if let Empty = self.get_cell_at(pos) {
                self.set_cell(pos, CellType::new_leaf());
                break;
            }
        }
    }

    fn column_update(column: &mut Vec<CellType>) {
        let mut current_energy = 1.;
        for cell in column {
            match cell {
                Empty | CellType::Seed => {}
                Leaf { sun_absorbed } => {
                    *sun_absorbed = current_energy * LEAF_ABSORB_RATE;
                    current_energy -= *sun_absorbed;
                }
                Trunk { .. } | CellType::Dead => {
                    current_energy -= TRUNK_ABSORB_RATE * current_energy;
                }

            }
        }
    }

    fn update_all_columns(&mut self) {
        self.cell_map
            .par_iter_mut()
            .for_each(|column| Habitat::column_update(column))
    }

    fn create_growths(&self) -> Vec<Vec<(IVec2, usize, CellType, f32)>> {
        // grow all plants
        let mut plant_growth: Vec<Vec<(IVec2, usize, CellType, f32)>> = Vec::with_capacity(self.plants.len());

        for plant in &self.plants {
            let mut growth_proposals = plant.create_growth_proposals();
            growth_proposals.retain(|(pos, _, _, _)|
                // check if growth doesn't leave y bounds
                self.is_in_y_bounds(*pos)
                    // check if growth position is not already occupied
                    && match self.get_cell_at(*pos) {
                    Empty => true,
                    _ => false
                }
            );
            let mut growths = Vec::with_capacity(MAX_GROWTHS_PER_ITERATION);

            for _ in 0..MAX_GROWTHS_PER_ITERATION {
                // if there are still growth proposals left...
                if growth_proposals.len() > 0 {
                    // ...add a random proposal to the growths, while removing it from the choice_pool
                    growths.push(growth_proposals.remove(gen_range(0, growth_proposals.len())));
                }
            }
            // add the vector of growths for the plant
            plant_growth.push(growths);
        }
        plant_growth
    }


    fn supply_plant_energy(&mut self) {
        let mut plant_leaves = Vec::<Vec<IVec2>>::new();

        // finding the leaf cells for each plant and storing them with position
        for mut plant in &mut self.plants {
            let mut leaves = Vec::<IVec2>::new();
            // iterate over all cells
            for (pos, cell) in plant.get_cells().iter() {
                match cell {
                    Leaf { .. } => {
                        leaves.push(IVec2::from(*pos));
                    }
                    _ => {}
                }
            }
            plant_leaves.push(leaves);
        }

        // going over the collected leaves of each plant and calculating the energy
        for (idx, leaves) in plant_leaves.iter().enumerate() {
            let mut collected_energy = 0f32;
            for leaf_pos in leaves {
                match self.get_cell_at(*leaf_pos) {
                    Leaf { sun_absorbed } => {
                        collected_energy += sun_absorbed * SUN_POWER;
                    }
                    _ => {}
                }
            }
            let energy_cost = self.plants[idx].get_cells().len() as f32 * CELL_SUSTAIN_ENERGY_COST;
            self.plants[idx].give_energy(
                collected_energy - energy_cost);
        }
    }

    fn update_seeds(&mut self) {
        // iterating over all seed indexes
        let mut indexes = Vec::<(usize, i32)>::with_capacity(self.plants.len());
        for ix in 0..self.seeds.len() {
            // updating the position to let the seed drop to 1 of the 3 lower cells
            let (x, y) = self.seeds[ix].get_pos().into();
            let new_pos = IVec2::new(x + gen_range(-1, 2), y-1);
            self.seeds[ix].set_pos(new_pos);
            // if the seed is now below ground it can potentially become a tree
            if let IVec2 { x, y: -1 } = self.seeds[ix].get_pos() {
                // setting seed position to ground level so it can grow a root
                self.seeds[ix].set_pos(IVec2::new(x + 1, 0));
                // saving seed index, to not mess up vector length
                indexes.push((ix, x));
            }
        }
        indexes.reverse();
        // starting with highest index first, to keep the ordering
        for (ix, x) in indexes {
            // moving the seed to ground buffer
            self.ground_buffer[((self.grid_size.x + x) % self.grid_size.x) as usize]
                .push(self.seeds.remove(ix));
        }
    }

    fn show_seeds(&mut self) {
        for ix in 0..self.seeds.len() {
            let pos = self.seeds[ix].get_pos();
            self.set_cell(pos, CellType::Seed);
        }
    }

    fn update_grounded_buffer(&mut self) {
        let mut growing_seeds = Vec::<(usize, usize)>::with_capacity(self.grid_size.x as usize);
        for (ix, container ) in self.ground_buffer.iter().enumerate() {
            // check if the ground tile at that position is free
            if let Empty = self.get_cell_at((ix as i32, 0i32).into()) {
                match container.len() {
                    0 => {}
                    1 => {
                        growing_seeds.push((ix, 0));
                    }
                    _ => {
                        // the plant with the highest amount of energy is allowed to grow
                        let container_ix = container.into_iter().enumerate()
                            .max_by(|(_, plant1), (_, plant2) |
                                plant1.get_energy().total_cmp(&plant2.get_energy())).unwrap().0;
                        growing_seeds.push((ix, container_ix));
                    }
                }
            }
        }
        // transfer transforming seeds
        for (column, ix) in growing_seeds {
            // remove from grounded seeds
            let mut plant = self.ground_buffer[column].remove(ix);
            // give it a root cell
            plant.create_root();
            // add the cell to plants
            self.plants.push(plant);
        }


        // reduce energy for remaining seeds
        for ix in 0..self.grid_size.x as usize {
            for plant in &mut self.ground_buffer[ix] {
                plant.give_energy(-SEED_ENERGY_DRAIN)
            }
            // remove all seeds which now have no energy
            self.ground_buffer[ix].retain(|seed| !seed.is_dead())
        }
    }
}

