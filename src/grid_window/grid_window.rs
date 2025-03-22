use macroquad::color::{BLACK, Color, WHITE};
use macroquad::math::{IVec2, Vec2};
use macroquad::shapes::{draw_line, draw_rectangle};
use ndarray::{Array, Ix2};
use rayon::iter::{IndexedParallelIterator};
use crate::constants::simulation::GRID_SIZE;

pub struct GridWindow {
    grid_pos: Vec2,
    grid_size: IVec2,
    grid_content: Array<Color, Ix2>,
    border_columns: (f32, f32),
    zoom: f32,
    pixel_dimensions: Vec2,
    x_offset: f32,
    default_cell_size: f32,
    cell_size: f32,
}

// draw a grid window with horizontal wrapping
impl GridWindow {
    pub fn new(grid_size: IVec2) -> Self {
        let mut x = Self {
            grid_pos: Default::default(),
            border_columns: Default::default(),
            grid_size,
            grid_content: Array::<Color, Ix2>::from_elem(
                (grid_size[0] as usize, grid_size[1] as usize), WHITE),
            zoom: 1f32,
            pixel_dimensions: Default::default(),
            x_offset: 0.001f32,
            default_cell_size: 20f32,
            cell_size: 20f32,
        };
        x.zoom(0f32);
        x
    }
    pub fn draw_grid(&self, pos: &Vec2, dim: &Vec2) {

        let adjusted_pos: Vec2 = (pos.x, pos.y
            + (dim.y - self.cell_size*self.grid_size.y as f32))
            .into();

        for i in 0..self.grid_size.y {
            draw_line(adjusted_pos.x, adjusted_pos.y + i as f32 * self.cell_size,
                      adjusted_pos.x + dim.x, adjusted_pos.y + i as f32 * self.cell_size,
                      1f32, BLACK);
        }
        let mut ix = 0;
        for i in 0..=(dim.x/self.cell_size) as i32 {
            let x_pos = adjusted_pos.x+self.x_offset + i as f32 * self.cell_size;
            if x_pos > dim.x + pos.x {break}
            draw_line(x_pos, adjusted_pos.y, x_pos, pos.y+dim.y,
                      match ix {0 => 3f32,_ => 1f32}, BLACK);
            ix = (ix + 1) % self.grid_size.x;
        }
        ix = self.grid_size.x;
        for i in 0..=(self.x_offset/self.cell_size) as i32 {
            let x_pos = adjusted_pos.x+self.x_offset - i as f32 * self.cell_size;
            if x_pos > dim.x + pos.x {continue}
            draw_line(x_pos, adjusted_pos.y, x_pos, pos.y+dim.y,
                      match ix {0 => 3f32,_ => 1f32}, BLACK);
            ix = (ix + 1) % self.grid_size.x;
        }

    }
    pub fn zoom(&mut self, x: f32) {
        self.zoom += x;
        // checks if current zoom would display the grid with size greater than y bounds
        if self.zoom * self.default_cell_size * self.grid_size.y as f32 > self.pixel_dimensions.y {
            self.zoom = self.pixel_dimensions.y / (self.default_cell_size * self.grid_size.y as f32)
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
    pub fn  draw_cells(&mut self, pos: &Vec2, dim: &Vec2) {

        // position where the actual grid starts (regarding zoom and grid_pos)
        let grid_pos = Vec2::new(
            pos.x,
            pos.y + (dim.y - self.cell_size*self.grid_size.y as f32));


        // width of the 2 chopped off columns
        self.border_columns = (
            self.x_offset % self.cell_size,
            (dim.x-self.x_offset) % self.cell_size
        );

        // amount of fully visible square columns plus the 2 chopped off
        let x_square_count = ((dim.x / self.cell_size).ceil()
            - ((self.border_columns.0+self.border_columns.1)/self.cell_size)) as usize + 2;

        //starting point for x index of cells
        let start_idx = (self.grid_size.x as f32 - (self.x_offset/self.cell_size).ceil()) as usize;

        //drawing all squares
        for y in 0..self.grid_size.y as usize {
            for cell_nbr in 0..x_square_count {
                //first chopped of columns
                if cell_nbr == 0 && self.x_offset != 0f32 {
                    draw_rectangle(
                        self.grid_pos.x,
                        self.grid_pos.y + y as f32*self.cell_size,
                        self.border_columns.0,
                        self.cell_size,
                        self.grid_content[[start_idx, y]]);
                    continue
                }
                let x = (start_idx + cell_nbr) % self.grid_size.x as usize;
                //second chopped off column
                if cell_nbr == x_square_count-1 && self.x_offset != 0f32 {
                    draw_rectangle(
                        self.grid_pos.x+self.border_columns.0 + (cell_nbr-1) as f32*self.cell_size,
                        self.grid_pos.y + y as f32*self.cell_size,
                        self.border_columns.1,
                        self.cell_size,
                        self.grid_content[[x, y]]);
                    continue
                }
                draw_rectangle(
                    self.grid_pos.x+self.border_columns.0 + (cell_nbr-1) as f32*self.cell_size,
                    self.grid_pos.y + y as f32*self.cell_size,
                    self.cell_size,
                    self.cell_size,
                    self.grid_content[[x, y]]);
            }
        }
    }

    pub fn draw_all(&mut self, pos: &Vec2, dim: &Vec2) {
        // save current dimensions to bound max zoom
        self.pixel_dimensions = *dim;
        draw_rectangle(pos.x, pos.y, dim.x, dim.y, WHITE);
        self.draw_cells(pos, dim);
        self.draw_grid(pos, dim)
    }

    pub fn world_to_grid(&self, grid_pos: &Vec2, grid_dim: &Vec2, pos: Vec2) -> Option<IVec2> {
        if pos.x < grid_pos.x || pos.y < self.grid_pos.y {return None}
        if pos.x > grid_pos.x + grid_dim.x || pos.y > grid_pos.y + grid_dim.y {return None}

        let grid_x = pos.x - self.grid_pos.x - self.x_offset;
        let grid_y = pos.y - self.grid_pos.y;

        let x = ((grid_x/(self.cell_size)) as i32 + self.grid_size.x) % self.grid_size.x;
        let y = (grid_y/(self.cell_size)).floor() as i32;

        Some(IVec2::new(x, y))
    }

    pub fn update_cells(&mut self, array: &[u8]) {
        assert_eq!(array.len(), (GRID_SIZE.x * GRID_SIZE.y * 3) as usize);

        let data = self.grid_content.as_slice_mut().unwrap();
        array.chunks(3).zip(data).for_each(|(i, color)| {
                *color = Color::from_rgba(i[0], i[1], i[2], 255);
        });
    }
}

