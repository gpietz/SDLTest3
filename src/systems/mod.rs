#![warn(clippy::pedantic)]

mod render_background;
mod render_bjarne;
mod render_text;
mod render_rocket;
mod movement_rocket;
mod movement_text;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
   Schedule::builder()
       .add_system(movement_rocket::movement_rocket_system())
       .add_system(movement_text::movement_text_system())
       .flush()
       .add_system(render_background::render_background_system())
       .add_system(render_bjarne::render_bjarne_system())
       .add_system(render_rocket::render_rocket_system())
       .add_system(render_text::render_text_system())
       .flush()
       .build()
}
