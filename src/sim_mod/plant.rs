use std::rc::Rc;
use crate::sim_mod::cell_types::CellType;

pub struct Plant {
    cells: Vec<((i32, i32), Rc<CellType>)>,
    root_pos: (i32, i32),
    root_connection: f32,
}

impl Plant {
    pub fn new(pos: (i32, i32)) -> Self {
        Self {
            cells: vec![(pos, Rc::new(CellType::new_root()))],
            root_pos: pos,
            root_connection: 1.,
        }
    }

    pub fn get_cells(&self) -> &Vec<((i32, i32), Rc<CellType>)> {
        &self.cells
    }

    pub fn make_iteration(&mut self) {
        let pos: (i32, i32) = self.cells.get(self.cells.len()-1).unwrap().0;
        self.cells.push(((pos.0, pos.1 + 1), Rc::new(CellType::Trunk {
            sun_blocked: 0.0,
            root_connection: 0.0,
        })));
    }
}