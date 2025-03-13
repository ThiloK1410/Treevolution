mod grid_window;
mod traits;
mod constants;
mod sim_mod;

use macroquad::input::KeyCode::*;
use macroquad::prelude::*;
use sim_mod::habitat::Habitat;
use crate::constants::simulation::GRID_SIZE;

pub const UPDATES_PER_SECOND: u32 = 60;
const TIME_PER_UPDATE: f32 = 1f32 / UPDATES_PER_SECOND as f32;


fn get_conf() -> Conf {

    Conf {
        window_title: "Treevolution".to_string(),
        window_width: 800,
        window_height: 600,
        high_dpi: false,
        fullscreen: true,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(get_conf())]
async fn main() {

    let mut grid = grid_window::GridWindow::new(
        Vec2::new(screen_width(), screen_height() / 2.),
        Vec2::new(0., screen_height() / 4.),
        GRID_SIZE);

    let mut habitat = Habitat::new(GRID_SIZE);

    let mut lag = 0.;
    let mut counter = 0;

    loop {
        counter += 1;
        if is_key_down(Escape) {break}

        lag += get_frame_time();

        while lag >= TIME_PER_UPDATE {
            lag -= TIME_PER_UPDATE;

            if is_key_down(Up) { grid.zoom(0.02f32) }
            if is_key_down(Down) { grid.zoom(-0.02f32) }
            if is_key_down(Left) { grid.change_offset(20f32) }
            if is_key_down(Right) { grid.change_offset(-20f32) }

            if is_key_down(Space) { habitat.spawn_random_cell() }
            if is_key_pressed(Enter) { habitat.spawn_plant() }

            if counter % 1 == 0 {
                habitat.update();
                counter = 0;
                if habitat.get_total_plant_count() < 30 {
                    habitat.spawn_plant()
                }
            }





            grid.update_cells(habitat.get_rgb_data().as_slice());
        }

        clear_background(GRAY);
        grid.draw_all();
        next_frame().await;

    }
}
