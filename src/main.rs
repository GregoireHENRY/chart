extern crate sdl2;
extern crate chrono;
extern crate dotenv;

mod settings;
mod toolbox;
mod chart;
use chart::candle;

use sdl2::event::Event;
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

fn main() -> Result<(), String>
{
    let _sdl2_test = Command::new("sdl2-config").arg("--version").output().is_err();
    //println!("{}", _sdl2_test);
    println!("{}", env!("LD_LIBRARY_PATH"));

    dotenv::dotenv().ok();
    let font = include_bytes!("../rsc/font/RobotoMono-Regular.ttf");
    let rwops_font = sdl2::rwops::RWops::from_bytes(font)?;

    let context = sdl2::init()?;
    let context_ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let video = context.video()?;
    let window = video.window("Chart", settings::env_u32("WIDTH"), settings::env_u32("HEIGHT")).position_centered().build().map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().accelerated().present_vsync().build().expect("Canvas build failed");
    let texture_creator = canvas.texture_creator();
    let mut font = context_ttf.load_font_from_rwops(rwops_font, 14)?;
    let mut events = context.event_pump()?;
    let mut _keyaltr: u16 = 0;

    let mut chart = chart::Chart::new();
    chart.add_candle(candle::Candle::new("2020 Apr 11 14:05:00".to_string() , 0.6,  0.5,  0.8,  0.4));
    chart.add_candle(candle::Candle::new("2020 Apr 11 14:00:00".to_string() , 0.3,  0.6,  0.7,  0.2));

    'wloop: loop {
        let state = events.mouse_state();
        chart.update_mouse(state.x(), state.y());
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'wloop,
                Event::KeyDown{keymod, ..} => _keyaltr = keymod.bits(),
                Event::KeyUp{keymod, ..} => _keyaltr = keymod.bits(),
                Event::MouseWheel {x, y, ..} => {
                    chart.scrolling(x, y);
                },
                _ => {}
            }
        }
        chart.update();
        canvas.clear();
        chart.draw(&mut canvas, &texture_creator, &mut font)?;
        canvas.present();
        sleep(Duration::from_millis(1000/60000));
    }

    Ok(())
}

/* ctrl: 4160
 */
