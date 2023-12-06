mod render;
mod intersection;
mod vehicle;

use crate::render::render;
use std::time::Duration;

use intersection::Intersection;
use sdl2::{ event::Event, keyboard::Keycode };

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Road", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let intersection = Intersection::new();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Update

        // Render
        render(&mut canvas, &intersection)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
