pub use crate::prelude::*;

use std::time::SystemTime;
use sdl2::render::WindowCanvas;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bjarne;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rocket(pub i8);

/////////////////////////////////////////////////////////////////////////////
// Image
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Image {
    pub texture_id: TextureId,
    pub position: Option<Point>,
    pub rotate: f64,
    pub flip: ImageFlip
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageFlip {
    None,
    Both,
    Horizontal,
    Vertical,
}

impl GetDrawCommand for Image {
    fn to_draw_command(&self) -> DrawCommand {
        DrawCommand::DrawImage {
            texture_id: self.texture_id,
            position: self.position,
            rotate: self.rotate,
            flip_horizontal: self.flip == ImageFlip::Both || self.flip == ImageFlip::Horizontal,
            flip_vertical: self.flip == ImageFlip::Both || self.flip == ImageFlip::Vertical
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// BouncingText
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct BouncingText(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BouncingTextMove {
    pub position: Point,
    pub target: Point,
    pub size: Option<Rect>,
    pub suspend_time: Option<SystemTime>
}

/////////////////////////////////////////////////////////////////////////////
// CanvasSize
/////////////////////////////////////////////////////////////////////////////


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanvasSize {
    pub width: u32,
    pub height: u32
}

impl CanvasSize {
    pub fn create_from(canvas: &WindowCanvas) -> Self {
        let (width, height) = canvas.output_size().unwrap();
        Self {
            width,
            height
        }
    }
}
