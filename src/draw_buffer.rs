use crate::prelude::*;

use std::sync::Mutex;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, TextureCreator, Texture, TextureQuery};
use sdl2::video::WindowContext;
use lazy_static::lazy_static;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

lazy_static! {
    static ref COMMAND_BUFFER: Mutex<Vec<(usize, Vec<DrawCommand>)>> =
        Mutex::new(Vec::with_capacity(100));
}

pub trait GetDrawCommand {
    fn to_draw_command(&self) -> DrawCommand;
}

#[derive(Debug)]
pub enum DrawCommand {
    ClearScreen,
    ClearColor {
        color: Color
    },
    DrawImage {
        texture_id: TextureId,
        position: Option<Point>,
        rotate: f64,
        flip_horizontal: bool,
        flip_vertical: bool
    },
    DrawText {
        text: String,
        position: Point
    }
}

pub struct DrawBatch {
    commands: Vec<DrawCommand>
}

impl DrawBatch {
    pub fn new() -> Self {
        Self {
            commands: Vec::new()
        }
    }

    pub fn submit(&mut self, z_order: usize) {
        let mut new_batch = Vec::with_capacity(self.commands.len());
        new_batch.append(&mut self.commands);
        COMMAND_BUFFER.lock().unwrap().push((z_order, new_batch));
    }

    pub fn add_cmd(&mut self, draw_command: DrawCommand) -> &mut Self {
        self.commands.push(draw_command);
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.commands.clear();
        self
    }

    pub fn render_draw_buffer(canvas: &mut WindowCanvas,
                              texture_creator: &TextureCreator<WindowContext>,
                              texture_map: &TextureMap,
                              font: &Font) {
        let mut command_buffer = COMMAND_BUFFER.lock().unwrap();
        command_buffer.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        command_buffer.iter().for_each(|(_, batch)| {
            batch.iter().for_each(|cmd| {
                match cmd {
                    DrawCommand::ClearScreen => canvas.clear(),
                    DrawCommand::ClearColor { color } => {
                        canvas.set_draw_color(color.rgb());
                        canvas.clear();
                    },
                    DrawCommand::DrawImage { texture_id, position, rotate, flip_horizontal, flip_vertical } => {
                        let texture = texture_map.get_texture(*texture_id).unwrap();
                        let mut target_rect = texture_map.get_texture_rect(*texture_id);
                        if position.is_none() {
                            DrawBatch::center_to_screen(canvas, &mut target_rect);
                        } else {
                            let pos = Point::from(position.unwrap());
                            target_rect.x = pos.x;
                            target_rect.y = pos.y;
                        }
                        canvas.copy_ex(texture, None, target_rect, *rotate, None, *flip_horizontal, *flip_vertical).unwrap();
                    },
                    DrawCommand::DrawText { text, position } => {
                        let text_texture = DrawBatch::create_text_texture(&texture_creator, font, text);
                        let mut rect = DrawBatch::create_centered_text_rect(canvas, &text_texture);
                        rect.x = position.x;
                        rect.y = position.y;
                        canvas.copy(&text_texture, None, rect).unwrap();
                        //println!("DrawText texture size = {}x{}", rect.width(), rect.height());
                    },
                }
            })
        });
        canvas.present();
        command_buffer.clear();
    }

    fn center_to_screen(canvas: &mut WindowCanvas, rect: &mut Rect) {
        let (width, height) = canvas.output_size().unwrap();
        rect.x = (width/2 - rect.width()/2) as i32;
        rect.y = (height/2 - rect.height()/2) as i32;
    }

    fn create_text_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>,
                           font: &Font,
                           text: &str) -> Texture<'a> {
        let surface = font
            .render(text)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())
            .unwrap();
        texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap()
    }

    fn create_centered_text_rect<'a>(canvas: &mut WindowCanvas, text_texture: &'a Texture) -> Rect {
        let TextureQuery { width, height, .. } = text_texture.query();
        let (screen_width, screen_height) = canvas.output_size().unwrap();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;
        DrawBatch::create_centered_rect(canvas,
            width,
            height,
            screen_width - padding,
            screen_height - padding,
        )
    }

    fn create_centered_rect(canvas: &mut WindowCanvas,
                            rect_width: u32,
                            rect_height: u32,
                            cons_width: u32,
                            cons_height: u32) -> Rect {
        let (screen_width, screen_height) = canvas.output_size().unwrap();

        let wr = rect_width as f32 / cons_width as f32;
        let hr = rect_height as f32 / cons_height as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                println!("Scaling down! The text will look worse!");
                let h = (rect_height as f32 / wr) as i32;
                (cons_width as i32, h)
            } else {
                println!("Scaling down! The text will look worse!");
                let w = (rect_width as f32 / hr) as i32;
                (w, cons_height as i32)
            }
        } else {
            (rect_width as i32, rect_height as i32)
        };

        let cx = (screen_width as i32 - w) / 2;
        let cy = (screen_height as i32 - h) / 2;
        rect!(cx, cy, w, h)
    }
}
