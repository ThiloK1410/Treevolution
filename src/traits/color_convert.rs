use macroquad::color::Color;

pub trait ColorConvert {
    fn get_color(&self) -> Color;
}