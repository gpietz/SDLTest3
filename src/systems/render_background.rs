use crate::prelude::*;
use sdl2::pixels::Color;

#[system]
pub fn render_background() {
    let mut draw_batch = DrawBatch::new();
    draw_batch.add_cmd(DrawCommand::ClearColor { color: Color::RGB(12,12,24)});
    draw_batch.submit(0);
}
