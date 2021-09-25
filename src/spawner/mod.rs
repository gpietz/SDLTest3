#![warn(clippy::pedantic)]

mod rocket_spawner;

pub use crate::spawner::rocket_spawner::*;

pub fn spawn_bjarne(ecs: &mut World) {
    ecs.push((Bjarne, Image {
        texture_id: TextureId::Bjarne,
        position: None,
        rotate: 0f64,
        flip: ImageFlip::None
    }));
}

pub fn spawn_bouncing_text(ecs: &mut World, text: &str) {
     ecs.push((BouncingText(text.into()), BouncingTextMove {
         position: Point::new(10,10),
         target: Point::new(10,10),
         size: None,
         suspend_time: None
     }));
}
