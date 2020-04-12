extern crate sdl2;

use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::rect::{Point, Rect};
use sdl2::ttf::Font;
use sdl2::pixels::Color;

pub fn map(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 { return x/(b-a)*(d-c); }
pub fn map_ax(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 { return d-(x-a)/(b-a)*(d-c); }

pub fn norm(p: Point) -> f32
{
    return ((p.x * p.x + p.y * p.y) as f32).sqrt();
}

pub fn unit(p: Point) -> Point
{
    return p / norm(p) as i32; 
}

pub fn position_text(x: u32, y: u32, w: u32, h: u32, align: &str) -> Rect
{
    let mut x = x as i32;
    let mut y = y as i32;
    let w = w as i32;
    let h = h as i32;
    match align {
        "right-top" => {
            x = x - w;
        },
        "right-mid" => {
            x = x - w;
            y = y - h/2;
        },
        "mid-top" => {
            x = x - w/2;
        },
        "mid-mid" => {
            x = x - w/2;
            y = y - h/2;
        }
        "left-mid" => {
            y = y - h/2;
        },
        _ => (),
    }
    Rect::new(x, y, w as u32, h as u32)
}

pub fn draw_dashed_line(canvas: &mut Canvas<Window>, p1: Point, pf: Point, n: i32) -> Result<(), String>
{
    let offset = unit(pf - p1).scale(2);
    let step = (pf - p1) / n;
    let mut p2 = p1;
    let mut p3;
    for _ in 0..n {
        p3 = p2 + step;
        canvas.draw_line(p2+offset, p3-offset)?;
        p2 = p3;
    }
    Ok(())
}

pub fn text(canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, font: &mut Font, s: &str, x: i32, y: i32, align: &str, color: Color) -> Result<(), String>
{
    let surface = font.render(s).blended(color).map_err(|e| e.to_string())?;
    let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
    let TextureQuery {width, height, ..} = texture.query();
    canvas.copy(&texture, None, position_text(x as u32, y as u32, width, height, align))?;
    Ok(())
}

pub fn rect(x: i32, y: i32, w: i32, h: i32, s: i32) -> Rect
{
    let mut rx = x;
    let mut ry = y;
    let mut rw = w;
    let mut rh = h;
    if w < 0 {
        rx +=  w;
        rw  = -rw;
    }
    if h < 0 {
        ry +=  h;
        rh  = -rh;
    }
    Rect::new(rx+s/2, ry+s/2, (rw-s) as u32, (rh-s) as u32)
}

pub fn draw_rect(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32) -> Result<(), String>
{
    canvas.fill_rect(rect(x, y, w, h, 0))?;
    Ok(())
}

pub fn rect_thick(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32, s: i32, co: Color, ci: Color) -> Result<(), String>
{
    canvas.set_draw_color(co);
    canvas.fill_rect(rect(x, y, w, h, 0))?;
    canvas.set_draw_color(ci);
    canvas.fill_rect(rect(x, y, w, h, s))?;
    Ok(())
}
