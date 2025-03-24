mod grid_window;
mod traits;
mod constants;
mod sim_mod;

use std::num::ParseIntError;
use std::str::FromStr;
use macroquad::hash;
use macroquad::input::KeyCode::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use sim_mod::habitat::Habitat;
use crate::constants::simulation::GRID_SIZE;
use crate::grid_window::GridWindow;

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
        window_resizable: true,
        icon: None,
        platform: Default::default(),
    }
}

enum UiFocus {
    None,
    NewGrid,
    MinPlants
}

#[macroquad::main(get_conf())]
async fn main() {

    let mut current_ui_focus = UiFocus::None;

    let mut grid_dim = Vec2::new(screen_width(), screen_height() / 2.);
    let mut grid_pos = Vec2::new(0., screen_height() / 4.);

    // responsible for drawing the habitat data
    let mut grid = GridWindow::new(GRID_SIZE);

    // responsible for simulation
    let mut habitat = Habitat::new(GRID_SIZE);
    habitat.set_minimum_plants((GRID_SIZE.x / 10) as usize);

    let mut lag = 0.;
    let mut counter = 0;
    let mut running = true;

    loop {
        counter += 1;
        if is_key_down(Escape) {break}

        lag += get_frame_time();

        while lag >= TIME_PER_UPDATE {
            lag -= TIME_PER_UPDATE;

            grid_dim = Vec2::new(screen_width(), screen_height() / 2.);
            grid_pos = Vec2::new(0., screen_height() / 4.);

            if is_key_down(Up) { grid.zoom(0.02f32) }
            if is_key_down(Down) { grid.zoom(-0.02f32) }
            if is_key_down(Left) { grid.change_offset(20f32) }
            if is_key_down(Right) { grid.change_offset(-20f32) }
            if is_key_pressed(Enter) { habitat.spawn_plant() }
            if is_key_pressed(Space) { running = !running }
            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(pos) =
                    grid.world_to_grid(&grid_pos, &grid_dim, mouse_position().into()) {
                    habitat.select_pos(pos);
                }
            }

            if running && counter % 1 == 0 {
                habitat.update();
                counter = 0;
            }


            grid.update_cells(habitat.get_rgb_data().as_slice());
        }

        clear_background(GRAY);
        draw_ui(&grid_pos,&mut current_ui_focus, &mut habitat, &mut grid);
        grid.draw_all(&grid_pos, &grid_dim);
        next_frame().await;

    }
    fn draw_ui(grid_pos: &Vec2, ui_focus: &mut UiFocus, habitat: &mut Habitat, grid_window: &mut GridWindow) {
        let padding = 10.;
        root_ui().window(hash!(),
                         vec2(padding, padding),
                         vec2(screen_width()/2.-padding*2., grid_pos.y-2.*padding), |ui| {
            ui.label(None, "settings");
            ui.separator();
            ui.same_line(20f32);
            if ui.button(None, "new grid") {
                *ui_focus = UiFocus::NewGrid;
            }
                ui.same_line(100f32);

                if ui.button(None, "set min plants") {
                    *ui_focus = UiFocus::MinPlants;
                }
        });

        root_ui().window(hash!(), vec2(padding, grid_pos.y+grid_window.get_dim().y),
                         vec2(screen_width()/2.-padding*2., grid_pos.y-2.*padding), |ui| {
                ui.label(None, "new Grid");

        });
    }
}
