extern crate sdl2;

use sdl2::pixels::Color;
use std::env;

pub const BLACK:  Color = Color::RGB(0, 0, 0);
pub const WHITE:  Color = Color::RGB(255, 255, 255);
pub const RED:    Color = Color::RGB(255, 0, 0);
pub const GREEN:  Color = Color::RGB(0, 255, 0);

pub fn env_f64(var: &str) -> f64 { env::var(var).expect(&format!("error loading environment variable: {}", var)).parse().unwrap() }
pub fn env_f32(var: &str) -> f32 { env_f64(var) as f32 }
pub fn env_i64(var: &str) -> i64 { env_f64(var) as i64 }
pub fn env_u32(var: &str) -> u32 { env_f64(var) as u32 }
