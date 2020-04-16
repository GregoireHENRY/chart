extern crate sdl2;
extern crate chrono;

use crate::settings;
use crate::toolbox;

use chrono::{DateTime, Utc, Duration};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::{Point, Rect};
use sdl2::ttf::Font;

pub struct Axis
{
    axe: i32,
    line: (Point, Point),
    tickpos: Vec<Point>,
    ticklabel: Vec<f32>,
    tick_size: Point,
    ticklabel_offset: Point,
    ticklabel_offset_0: Point,
    ticklabel_offset_f: Point,
    pub max: f32,
    pub min: f32,
    pub scroll: f32,
    zoom: f32,
    ntick: usize,
    align: String,
    mir: Rect, //mouse indicator
}

impl Axis
{
    pub fn new(axe: i32) -> Axis
    {
        let ntick = 5;
        let max;
        let min;
        let line;
        let tickpos = vec![Point::new(0, 0); ntick];
        let ticklabel = vec![0.0; ntick];
        let tick_size;
        let ticklabel_offset;
        let ticklabel_offset_0;
        let ticklabel_offset_f;
        let align;
        let mir;
        if axe == 0 {
            max = 0.0;
            min = -3600.0;
            line = (Point::new(0, settings::env_u32("HEIGHT") as i32-100), Point::new(settings::env_u32("WIDTH") as i32, settings::env_u32("HEIGHT") as i32-100));
            tick_size = Point::new(0, 5);
            ticklabel_offset = Point::new(0, 20);
            ticklabel_offset_0 = Point::new(50, 0);
            ticklabel_offset_f = Point::new(-150, 0);
            align = "mid-top";
            mir = Rect::new(0, settings::env_u32("HEIGHT") as i32-100, 100, 20);
        }
        else {
            max = 1.0;
            min = -1.0;
            line = (Point::new(settings::env_u32("WIDTH") as i32-100, settings::env_u32("HEIGHT") as i32), Point::new(settings::env_u32("WIDTH") as i32-100, 0));
            tick_size = Point::new(5, 0);
            ticklabel_offset = Point::new(20, 0);
            ticklabel_offset_0 = Point::new(0,-150);
            ticklabel_offset_f = Point::new(0, 50);
            align = "left-mid";
            mir = Rect::new(settings::env_u32("WIDTH") as i32-100, 0, 100, 20);
        }
        Axis{axe: axe,
             line: line,
             tickpos: tickpos,
             ticklabel: ticklabel,
             tick_size: tick_size,
             ticklabel_offset: ticklabel_offset,
             ticklabel_offset_0: ticklabel_offset_0,
             ticklabel_offset_f: ticklabel_offset_f,
             max: max,
             min: min,
             scroll: 0.0,
             zoom: 0.0,
             ntick: ntick,
             align: align.to_string(),
             mir: mir}
    }
    pub fn update(&mut self)
    {
        self.update_ticks();
    }
    fn update_ticks(&mut self)
    {
        let n = self.ntick;
        self.ticklabel[0] = self.min;
        self.ticklabel[n-1] = self.max;
        self.tickpos[0] = self.line.0+self.ticklabel_offset_0;
        self.tickpos[n-1] = self.line.1+self.ticklabel_offset_f;
        let step_pos = (self.tickpos[n-1] - self.tickpos[0]) / (n - 1) as i32;
        let step_label = (self.ticklabel[n-1] - self.ticklabel[0]) / (n - 1) as f32;
        for i in 1..n-1 {
            self.tickpos[i] = self.tickpos[i-1] + step_pos;
            self.ticklabel[i] = self.ticklabel[i-1] + step_label;
        }
    }
    pub fn scrolling(&mut self, val: f32)
    {
        let scroll = val * (self.max - self.min) * 0.05;
        self.scroll += scroll;
        self.min += scroll;
        self.max += scroll;
    }
    pub fn zooming(&mut self, val: f32, target: f32)
    {
        let zoom = val * (self.max - self.min) * 0.05;
        if (self.zoom > 150.0 && val > 0.0) || (self.zoom < -50.0 && val < 0.0) { return; }
        self.zoom += val;
        if (target - self.min).abs() < (target - self.max).abs() {
            if val > 0.0 { self.max -= zoom; }
            else         { self.min += zoom; }
        }
        else { 
            if val > 0.0 { self.min += zoom; }
            else         { self.max -= zoom; }
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, font: &mut Font, time: DateTime<Utc>) -> Result<(), String>
    {
        canvas.set_draw_color(settings::BLACK);
        if self.axe == 0 {toolbox::draw_rect(canvas, 0, settings::env_u32("HEIGHT") as i32-100, settings::env_u32("WIDTH") as i32-100, 100)?; }
        else             {toolbox::draw_rect(canvas, settings::env_u32("WIDTH") as i32-100, 0,  100, settings::env_u32("HEIGHT") as i32-100)?; }
        canvas.set_draw_color(settings::WHITE);
        canvas.draw_line(self.line.0, self.line.1)?;
        self.draw_ticklabel(canvas, texture_creator, font, time)?;
        Ok(())
    }
    fn draw_ticklabel(&self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, font: &mut Font, time: DateTime<Utc>) -> Result<(), String>
    {
        let size = self.tick_size;
        let off = self.ticklabel_offset;
        for i in 0..self.ntick {
            let pos = self.tickpos[i];
            canvas.draw_line(pos-size, pos+size)?;
            let label;
            if self.axe == 1 { 
                label = format!("{:8.5}", self.ticklabel[i]);
            }
            else {
                let ticklabel = time + Duration::seconds(self.ticklabel[i] as i64) + Duration::seconds(settings::env_i64("GMT") as i64 *3600);
                let ticklabel2: DateTime<Utc> = ticklabel.into();
                label = format!("{}", ticklabel2.format("%R"));
            }
            toolbox::text(canvas, texture_creator, font, &label, pos.x+off.x, pos.y+off.y, &self.align, settings::WHITE)?;
        }
        Ok(())
    }
    pub fn draw_mouse_indicator(&mut self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, font: &mut Font, x: i32, time: DateTime<Utc>) -> Result<(), String>
    {
        let string;
        if self.axe == 0 {
            self.mir.x = x - self.mir.w/2;
            let data = toolbox::map_ax(x as f32, 50.0, settings::env_u32("WIDTH") as f32-150.0, self.max, self.min);
            let ticklabel = time + Duration::seconds(data as i64) + Duration::seconds(settings::env_i64("GMT") *3600);
            let ticklabel2: DateTime<Utc> = ticklabel.into();
            string = format!("{}", ticklabel2.format("%R"));
        }
        else {
            self.mir.y = x - self.mir.h/2;
            string = format!("{:8.5}", toolbox::map_ax(x as f32, 50.0, settings::env_u32("HEIGHT") as f32-150.0, self.min, self.max)); 
        }
        canvas.fill_rect(self.mir)?;
        toolbox::text(canvas, texture_creator, font, &string, self.mir.center().x, self.mir.center().y, "mid-mid", settings::BLACK)?;
        Ok(())
    }
}
