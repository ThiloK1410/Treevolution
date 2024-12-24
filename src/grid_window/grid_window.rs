use macroquad::color::{BLACK, Color, WHITE};
use macroquad::math::{IVec2, Vec2};
use macroquad::shapes::{draw_line, draw_rectangle};
use ndarray::{Array, Ix2};
use crate::traits::color_convert::ColorConvert;

pub struct GridWindow {
    dim: Vec2,
    pos: Vec2,
    grid_size: IVec2,
    grid_content: Array<Color, Ix2>,
    zoom: f32,
    x_offset: f32,
    default_cell_size: f32,
    cell_size: f32,
}

// draw a grid window with horizontal wrapping
impl GridWindow {
    pub fn new(dim: Vec2, pos: Vec2, grid_size: IVec2) -> Self {
        let mut x = Self {
            dim,
            pos,
            grid_size,
            grid_content: Array::<Color, Ix2>::from_elem(
                (grid_size[0] as usize, grid_size[1] as usize), WHITE),
            zoom: 1f32,
            x_offset: 0.001f32,
            default_cell_size: 20f32,
            cell_size: 20f32,
        };
        x.zoom(0f32);
        x
    }
    pub fn draw_grid(&self) {

        let adjusted_pos: Vec2 = (self.pos.x, self.pos.y
            + (self.dim.y - self.cell_size*self.grid_size.y as f32))
            .into();

        for i in 0..self.grid_size.y {
            draw_line(adjusted_pos.x, adjusted_pos.y + i as f32 * self.cell_size,
                      adjusted_pos.x + self.dim.x, adjusted_pos.y + i as f32 * self.cell_size,
                      1f32, BLACK);
        }
        let mut ix = 0;
        for i in 0..=(self.dim.x/self.cell_size) as i32 {
            let x_pos = adjusted_pos.x+self.x_offset + i as f32 * self.cell_size;
            if x_pos > self.dim.x + self.pos.x {break}
            draw_line(x_pos, adjusted_pos.y, x_pos, self.pos.y+self.dim.y,
                      match ix {0 => 3f32,_ => 1f32}, BLACK);
            ix = (ix + 1) % self.grid_size.x;
        }
        ix = self.grid_size.x;
        for i in 0..=(self.x_offset/self.cell_size) as i32 {
            let x_pos = adjusted_pos.x+self.x_offset - i as f32 * self.cell_size;
            if x_pos > self.dim.x + self.pos.x {continue}
            draw_line(x_pos, adjusted_pos.y, x_pos, self.pos.y+self.dim.y,
                      match ix {0 => 3f32,_ => 1f32}, BLACK);
            ix = (ix + 1) % self.grid_size.x;
        }

    }
    pub fn zoom(&mut self, x: f32) {
        self.zoom += x;
        // checks if current zoom would display the grid with size greater than y bounds
        if self.zoom * self.default_cell_size * self.grid_size.y as f32 > self.dim.y {
            self.zoom = self.dim.y / (self.default_cell_size * self.grid_size.y as f32)
        }
        if self.zoom < 0.2f32 {
            self.zoom = 0.2f32;
        }
        self.cell_size = self.default_cell_size * self.zoom;
    }
    pub fn change_offset(&mut self, x: f32) {
        self.x_offset += x;
        if self.x_offset < 0f32 {
            self.x_offset += self.cell_size * self.grid_size.x as f32;
        }
        if self.x_offset > self.cell_size * self.grid_size.x as f32 {
            self.x_offset -= self.cell_size * self.grid_size.x as f32;
        }
    }
    pub fn draw_cells(&self) {

        // position where the actual grid starts (regarding zoom and grid_pos)
        let grid_pos = Vec2::new(
            self.pos.x,
            self.pos.y + (self.dim.y - self.cell_size*self.grid_size.y as f32));


        // width of the 2 chopped off columns
        let cell_width1 = self.x_offset % self.cell_size;
        let cell_width2 = (self.dim.x-self.x_offset) % self.cell_size;

        // amount of fully visible square columns plus the 2 chopped off
        let x_square_count = ((self.dim.x / self.cell_size).ceil()
            - ((cell_width1+cell_width2)/self.cell_size)) as usize + 2;

        //starting point for x index of cells
        let start_idx = (self.grid_size.x as f32 - (self.x_offset/self.cell_size).ceil()) as usize;

        //drawing all squares
        for y in 0..self.grid_size.y as usize {
            for cell_nbr in 0..x_square_count {
                //first chopped of columns
                if cell_nbr == 0 && self.x_offset != 0f32 {
                    draw_rectangle(
                        grid_pos.x,
                        grid_pos.y + y as f32*self.cell_size,
                        cell_width1,
                        self.cell_size,
                        self.grid_content[[start_idx, y]]);
                    continue
                }
                let x = (start_idx + cell_nbr) % self.grid_size.x as usize;
                //second chopped off column
                if cell_nbr == x_square_count-1 && self.x_offset != 0f32 {
                    draw_rectangle(
                        grid_pos.x+cell_width1 + (cell_nbr-1) as f32*self.cell_size,
                        grid_pos.y + y as f32*self.cell_size,
                        cell_width2,
                        self.cell_size,
                        self.grid_content[[x, y]]);
                    continue
                }
                draw_rectangle(
                    grid_pos.x+cell_width1 + (cell_nbr-1) as f32*self.cell_size,
                    grid_pos.y + y as f32*self.cell_size,
                    self.cell_size,
                    self.cell_size,
                    self.grid_content[[x, y]]);
            }
        }
    }

    pub fn draw_all(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.dim.x, self.dim.y, WHITE);
        self.draw_cells();
        self.draw_grid()
    }

    pub fn update_cells(&mut self, array: Array<impl ColorConvert, Ix2>) {
        assert_eq!(array.shape(), self.grid_content.shape());

        self.grid_content = array.map(ColorConvert::get_color);
    }
}

