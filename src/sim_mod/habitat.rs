use std::collections::HashMap;
use std::rc::Rc;
use macroquad::math::IVec2;
use ndarray::{Array, Ix2};
use rand::Rng;
use crate::sim_mod::cell_types::CellType;
use crate::sim_mod::cell_types::CellType::{Empty, Leaf, Trunk};
use crate::sim_mod::plant::Plant;
use crate::constants::simulation::{LEAF_ABSORB_RATE, TRUNK_ABSORB_RATE};

// control struct, to hold the data of every tree and information of whole grid
pub struct Habitat {
    grid_size: IVec2,
    cell_map: Array<Rc<CellType>, Ix2>,
    plants: Vec<Plant>,
    sun_power: f32,
}

impl Habitat {
    pub fn new(grid_size: IVec2) -> Self{
        Self {
            grid_size,
            cell_map: Array::<Rc<CellType>, Ix2>::from_elem(
                (grid_size[0] as usize, grid_size[1] as usize),
                Rc::new(Empty)
            ),
            plants: Vec::new(),
            sun_power: 10f32,
        }
    }

    fn apply_plants(&mut self) {
        let mut cells: HashMap<(i32, i32), Rc<CellType>> = HashMap::new();
        for plant in &self.plants {
            for (pos, cell) in plant.get_cells().iter() {
                let transformed_pos =
                    (pos.0 % self.grid_size.x, self.grid_size.y - 1 - pos.1);
                if self.is_in_grid(transformed_pos) {
                cells.entry(
                    // inverting y pos and wrapping x around
                    transformed_pos)
                    .or_insert(cell.clone());
                    }
            }
        }
        for (pos, cell) in cells.iter() {
            self.set_cell((*pos).into(), cell.clone())
        }
    }

    pub fn spawn_plant(&mut self) {
        let mut rng = rand::rng();
        let pos = (rng.random_range(0..self.grid_size.x),
                             0);
        if let Empty = self.get_cell(pos.into()).as_ref() {
            self.plants.push(Plant::new(pos));
        }
    }

    pub fn update(&mut self) {
        for mut plant in &mut self.plants {
            plant.make_iteration()
        }
        self.apply_plants();
        self.update_all();
    }

    pub fn make_test_fill(&mut self) {
        for x in 0..self.grid_size.x {
            let pos = IVec2::new(x, x % self.grid_size.y);
            self.set_cell(pos, Rc::new(Trunk { sun_blocked: 0f32, root_connection: 0f32}))
        }
    }

    pub fn set_cell(&mut self, pos: IVec2, cell_type: Rc<CellType>) {
        self.cell_map[[pos.x as usize, pos.y as usize]] = cell_type;
    }
    pub fn get_cell(&self , pos: IVec2) -> Rc<CellType> {
        self.cell_map[[pos.x as usize, pos.y as usize]].clone()
    }
    pub fn get_cell_map(&self) -> &Array<Rc<CellType>, Ix2> {
        &self.cell_map
    }

    pub fn is_in_grid(&self, pos: (i32, i32)) -> bool {
        !(pos.0<0 || pos.0 >= self.grid_size.x || pos.1<0 || pos.1 >= self.grid_size.y)
    }


    pub fn spawn_random_cell(&mut self) {
        let mut rng = rand::rng();
        // try to place a cell at random position, 100 tries at max
        for _ in 0..100 {
            let pos = IVec2::new(rng.random_range(0..self.grid_size.x),
                                 rng.random_range(0..self.grid_size.y));
            if let Empty = self.get_cell(pos).as_ref() {
                self.set_cell(pos, Rc::new(CellType::new_leaf()));
                break;
            }
        }
    }
    pub fn column_update(&mut self, column: usize) {
        let mut current_energy = 1.;
        for y in 0..self.grid_size.y as usize {
            match &*self.cell_map[[column, y]] {
                Leaf { .. } => {
                    let x = current_energy * LEAF_ABSORB_RATE;
                    self.cell_map[[column, y]] = Rc::new(Leaf { sun_absorbed: x});
                    current_energy -= x;
                }
                Trunk { root_connection, .. } => {
                    let x = current_energy * TRUNK_ABSORB_RATE;
                    self.cell_map[[column, y]] =
                        Rc::new(Trunk { sun_blocked: x, root_connection: *root_connection});
                    current_energy -= x;
                }
                Empty => (),
            }
        }
    }

    pub fn update_all(&mut self) {
        for x in 0..self.grid_size.x as usize {
            self.column_update(x);
        }
    }
}