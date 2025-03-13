use macroquad::math::IVec2;
use crate::sim_mod::cell_types::CellType;

pub struct PlantCell {
    pos: IVec2,
    cell_type: CellType,
    response_ix: usize
}

impl PlantCell {
    pub fn new(pos: IVec2, cell_type: CellType, response_ix: usize) -> Self {
        Self {
            pos: pos.into(),
            cell_type,
            response_ix
        }
    }


    pub fn new_root(pos: IVec2) -> Self {
        Self {
            pos,
            cell_type: CellType::new_root(),
            response_ix: 0,
        }
    }

    pub fn get_pos(&self) -> IVec2 {
        self.pos
    }

    pub fn get_cell_type(&self) -> CellType {
        self.cell_type
    }

    pub fn get_response_ix(&self) -> usize {
        self.response_ix
    }
}
