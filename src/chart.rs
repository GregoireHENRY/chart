extern crate sdl2;
extern crate chrono;

mod axis;
pub mod candle;

use crate::settings;
use crate::toolbox;

use chrono::{DateTime, Utc};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::{Point, Rect};
use sdl2::ttf::Font;

pub struct Chart
{
    xaxis: axis::Axis,
    yaxis: axis::Axis,
    mouseh: (Point, Point), 
    mousev: (Point, Point),
    time: DateTime<Utc>,
    candles: Vec<candle::Candle>,
}

impl Chart
{
    pub fn new() -> Chart {
        Chart{xaxis: axis::Axis::new(0),
              yaxis: axis::Axis::new(1),
              mouseh: (Point::new(0, 0), Point::new(settings::env_u32("WIDTH") as i32, 0)),
              mousev: (Point::new(0, 0), Point::new(0, settings::env_u32("HEIGHT") as i32)),
              time: Utc::now(),
              candles: Vec::new()}
    }
    pub fn add_candle(&mut self, candle: candle::Candle)
    {
        self.candles.push(candle);
    }
    pub fn update(&mut self)
    {
        self.time = Utc::now();
        self.xaxis.update();
        self.yaxis.update();
    }
    pub fn update_mouse(&mut self, x: i32, y: i32)
    {
        self.mouseh.0.y = y;
        self.mouseh.1.y = y;
        self.mousev.0.x = x;
        self.mousev.1.x = x;
    }
    pub fn scrolling(&mut self, x: i32, y: i32)
    {
        let target;
        if self.candles.len() > 0 { target = self.candles[0].get_middle(); }
        else                      { target = 0.0; }
        self.xaxis.scrolling(-x as f32);
        self.yaxis.zooming(y as f32, target);
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, font: &mut Font) -> Result<(), String>
    {
        canvas.set_draw_color(settings::BLACK);
        canvas.fill_rect(Rect::new(0, 0, settings::env_u32("WIDTH"), settings::env_u32("HEIGHT")))?;
        self.draw_candles(canvas)?;
        self.xaxis.draw(canvas, texture_creator, font, self.time)?;
        self.yaxis.draw(canvas, texture_creator, font, self.time)?;
        self.draw_mouse(canvas, texture_creator, font)?;
        Ok(())
    }
    fn draw_mouse(&mut self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, font: &mut Font) -> Result<(), String>
    {
        if self.mousev.0.x <= 0 || self.mousev.0.x > settings::env_u32("WIDTH") as i32-100 || self.mouseh.0.y <= 0 || self.mouseh.0.y > settings::env_u32("HEIGHT") as i32-100 {
            return Ok(());
        }
        canvas.set_draw_color(settings::WHITE);
        toolbox::draw_dashed_line(canvas, self.mouseh.0, self.mouseh.1, 100)?;
        toolbox::draw_dashed_line(canvas, self.mousev.0, self.mousev.1, 100)?;
        self.xaxis.draw_mouse_indicator(canvas, texture_creator, font, self.mousev.0.x, self.time)?;
        self.yaxis.draw_mouse_indicator(canvas, texture_creator, font, self.mouseh.0.y, self.time)?;
        Ok(())
    }
    fn draw_candles(&self, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        for candle in self.candles.iter() {
            candle.draw(canvas, &self.xaxis, &self.yaxis, self.time)?;
        }
        Ok(())
    }
}
