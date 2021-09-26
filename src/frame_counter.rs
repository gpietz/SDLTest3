use crate::prelude::*;

use sdl2::*;

pub struct FrameCounter  {
    ticks: u32
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            ticks: 0
        }
    }

    pub fn start(&mut self, sdl_context: &Sdl) {
        self.ticks = sdl_context.timer().unwrap().ticks();
    }

    pub fn get_elapsed_seconds(&self, sdl_context: &Sdl) -> f64 {
        let ticks = sdl_context.timer().unwrap().ticks();
        (ticks - self.ticks) as f64 / 1000.0f64
    }

    pub fn get_frame_count(&self, sdl_context: &Sdl) -> f64 {
        let elapsed_seconds = self.get_elapsed_seconds(&sdl_context);
        1.0f64 / elapsed_seconds
    }
}
