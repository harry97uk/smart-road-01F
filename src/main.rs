mod render;
mod intersection;
mod vehicle;
mod physics;
mod algorithm;

use crate::render::render;
use std::time::{ Duration, Instant };

use intersection::Intersection;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{ TextureCreator, Texture },
    video::WindowContext,
    image::LoadTexture,
};

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 600;
const KEY_PRESS_INTERVAL: Duration = Duration::from_millis(200);
const SPAWN_INTERVAL: Duration = Duration::from_millis(800);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Road", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();
    let car_texture = create_car_texture(&texture_creator);
    let road_texture = create_road_texture(&texture_creator);

    let mut last_keypress_time = Instant::now();
    let mut last_spawn_time = Instant::now();

    let mut intersection = Intersection::new();

    let mut continuous_spawning = false;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    let elapsed_time = Instant::now().duration_since(last_keypress_time);
                    if elapsed_time >= KEY_PRESS_INTERVAL {
                        match keycode {
                            Keycode::Left | Keycode::Right | Keycode::Up | Keycode::Down => {
                                intersection.add_directed_vehicle(keycode);
                            }
                            Keycode::R => {
                                continuous_spawning = !continuous_spawning;
                            }
                            _ => {}
                        }
                        last_keypress_time = Instant::now();
                    }
                }

                _ => {}
            }
        }

        // Update
        let elapsed_spawn_time = Instant::now().duration_since(last_spawn_time);
        if continuous_spawning && elapsed_spawn_time >= SPAWN_INTERVAL {
            intersection.add_random_vehicle();
            last_spawn_time = Instant::now();
        }
        intersection.update();

        // Render
        render(&mut canvas, &intersection, &car_texture, &road_texture)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn create_car_texture(texture_creator: &TextureCreator<WindowContext>) -> Texture {
    let path = format!("src/assets/179664-OWO44A-16-removebg-preview.png");
    texture_creator.load_texture(&path).expect(&format!("could not load texture: {}", path))
}

fn create_road_texture(texture_creator: &TextureCreator<WindowContext>) -> Texture {
    let path = format!("src/assets/2112_w032_n003_284b_p1_284.jpg");
    texture_creator.load_texture(&path).expect(&format!("could not load texture: {}", path))
}
