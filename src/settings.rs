extern crate sdl2;

use sdl2::pixels::Color;

pub const WIDTH:  u32   = 800;
pub const HEIGHT: u32   = 600;
pub const BLACK:  Color = Color::RGB(0, 0, 0);
pub const WHITE:  Color = Color::RGB(255, 255, 255);
pub const RED:    Color = Color::RGB(255, 0, 0);
pub const GREEN:  Color = Color::RGB(0, 255, 0);
pub const PERIOD: i32   = 5*60;
pub const GMT:    i32   = 2;
