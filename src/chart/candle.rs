extern crate sdl2;
extern crate chrono;

use crate::settings;
use crate::toolbox;
use crate::chart::axis;

use chrono::{DateTime, FixedOffset, Utc};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Point;

pub struct Candle
{
    time: DateTime<FixedOffset>,
    open: f32,
    close: f32,
    high: f32,
    low: f32,
}

impl Candle
{
    pub fn new(timestr: String, open: f32, close: f32, high: f32, low: f32) -> Candle
    {
        let time = DateTime::parse_from_str(&format!("{}{}", timestr, " +0000"), "%Y %b %d %H:%M:%S %z").unwrap();
        Candle{time: time, open: open, close: close, high: high, low: low}
    }
    pub fn get_middle(&self) -> f32
    {
        return (self.open+self.close)/2.0;
    }
    fn get_side(&self) -> bool
    {
        return self.close > self.open;
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>, xaxis: &axis::Axis, yaxis: &axis::Axis, time: DateTime<Utc>) -> Result<(), String>
    {
        let xmin = xaxis.min;
        let xmax = xaxis.max;
        let ymin = yaxis.min;
        let ymax = yaxis.max;
        let sec = time.signed_duration_since(self.time).num_seconds();
        let wi = 2;
        let w = toolbox::map(settings::PERIOD as f32, xmin, xmax, 0.0, 600.0) as i32 -2*wi;
        let x = toolbox::map_ax(-sec as f32, xmax, xmin, 50.0, settings::WIDTH as f32-150.0) as i32; 
        let h = toolbox::map_ax(self.high, ymin, ymax, 50.0, settings::HEIGHT as f32-150.0) as i32;
        let l = toolbox::map_ax(self.low, ymin, ymax, 50.0, settings::HEIGHT as f32-150.0) as i32;
        let o = toolbox::map_ax(self.open, ymin, ymax, 50.0, settings::HEIGHT as f32-150.0) as i32;
        let c = toolbox::map_ax(self.close, ymin, ymax, 50.0, settings::HEIGHT as f32-150.0) as i32;
        let ci;
        if  x < -w || x > settings::WIDTH as i32-100+w || (self.get_side() && self.low < 0.0) || (!self.get_side() && self.high > settings::HEIGHT as f32-100.0) { return Ok(()); }
        if o > c { ci = settings::GREEN; }
        else     { ci = settings::RED; }
        canvas.set_draw_color(settings::WHITE);
        canvas.draw_line(Point::new(x, h), Point::new(x, l))?;
        toolbox::rect_thick(canvas, x-w/2, o, w, c-o, wi, settings::WHITE, ci)?;
        Ok(())
    }
}
