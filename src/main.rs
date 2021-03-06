#![warn(clippy::pedantic)]

use std::time::{SystemTime, Duration};

use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;

use prelude::*;

use crate::frame_counter::*;
use crate::texture_map::TextureMap;

mod soundapi;
mod systems;
mod components;
mod texture_map;
mod draw_buffer;
mod spawner;
mod frame_counter;

mod prelude {
    // Extern
    pub use legion::*;
    pub use legion::query::*;
    pub use legion::systems::{CommandBuffer, Resources};
    pub use legion::world::SubWorld;
    pub use sdl2::rect::Point;
    pub use sdl2::rect::Rect;
    pub use sdl2::ttf::Font;

    // Misc
    pub use crate::components::*;
    pub use crate::draw_buffer::*;
    pub use crate::DrawCommand::*;
    pub use crate::soundapi::*;
    // Mods
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::texture_map::*;

    // Constants
    pub static WINDOW_TILE: &str = "SDL Test ]|[";
    pub const SCREEN_WIDTH: u32 = 1024;
    pub const SCREEN_HEIGHT: u32 = 768;
    pub const SPACESHIP_WIDTH: u32 = 64;
    pub const SPACESHIP_HEIGHT: u32 = 29;
}

fn main() {
    // Initialize SDL systems
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    // Create window, drawing canvas and texture creator
    let window = video_subsystem.window(WINDOW_TILE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();

    // Load textures
    let mut texture_map = TextureMap::new();
    texture_map.load_textures(&texture_creator);

    // Load font
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttf_context.load_font("resources/fonts/visitor2.ttf", 48).unwrap();

    // Create eco system
    let mut world = World::default();
    let mut resources = Resources::default();

    // Initialize resources
    resources.insert(CanvasSize::create_from(&canvas));

    spawn_bjarne(&mut world);
    spawn_bouncing_text(&mut world, "Bjarne Stress Test");

    let mut rocket_spawner = RocketSpawner::new();
    let mut systems = build_scheduler();

    // Start music
    let mut sound_api = SoundApi::new();
    sound_api.play_streamed("resources/sfx/music/Juhani_Junkala-Stage2.ogg");
    sound_api.set_stream_looped(true);

    let delay_time = Duration::new(0, 100000000 / 10);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut fps_count = FrameCounter::new();
    let mut window_updated = SystemTime::now();
    'main: loop {
        fps_count.start(&sdl_context);

        // Even polling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'main,
                    _ => {}
                }
                _ => {},
            }
        }

        rocket_spawner.spawn(&mut world, &mut canvas);
        systems.execute(&mut world, &mut resources);

        // Render
        DrawBatch::render_draw_buffer(&mut canvas, &texture_creator, &texture_map, &font);

        // Update resources
        resources.insert(CanvasSize::create_from(&canvas));

        // Get fps count
        let fps = fps_count.get_frame_count(&sdl_context);

        // Update window title with rocket count
        if window_updated.elapsed().unwrap().as_secs() >= 1 {
            window_updated = SystemTime::now();
            let rocket_count = <&Rocket>::query().iter(&world).count();
            let mut new_window_title = WINDOW_TILE.to_owned();
            new_window_title.push_str(&format!("  -> Rockets #{} // FPS: {}", rocket_count, fps as u32));
            canvas.window_mut().set_title(new_window_title.as_str()).unwrap();
        }

        // Delay
        std::thread::sleep(delay_time);
    }
}
