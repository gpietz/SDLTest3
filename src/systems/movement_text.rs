use crate::prelude::*;

use std::time::{SystemTime, Duration};
use rand::distributions::{Distribution, Uniform};
use std::ops::Add;
use rand::Rng;

// TODO Argh, constants are bad !!!!
const TEXTURE_WIDTH  : i32 = 396;
const TEXTURE_HEIGHT : i32 = 34;
const CANVAS_PADDING : i32 = 10;

#[system]
#[write_component(BouncingTextMove)]
pub fn movement_text(ecs: &mut SubWorld,
                     #[resource] canvas_size: &CanvasSize) {
    let mut text_move = <&mut BouncingTextMove>::query().filter(component::<BouncingText>())
        .iter_mut(ecs)
        .nth(0)
        .unwrap();

    if text_move.position == text_move.target {
        if text_move.suspend_time.is_none() {
            let mut rng = rand::thread_rng();
            let suspend_time = rng.gen_range(0..=4);
            text_move.suspend_time = Some(SystemTime::now().add(Duration::from_secs(suspend_time)));
            return;
        } else if text_move.suspend_time.is_some()
            && text_move.suspend_time.unwrap() > SystemTime::now() {
            return;
        }

        text_move.suspend_time = None;

        let canvas_half_width = (canvas_size.width/2) as i32;
        let canvas_half_height = (canvas_size.height/2) as i32;

        let min_x: i32;
        let max_x: i32;
        let min_y: i32;
        let max_y: i32;

        if text_move.position.x < canvas_half_width as i32 {
            min_x = canvas_half_width as i32;
            max_x = canvas_size.width as i32 - CANVAS_PADDING;
        } else {
            min_x = CANVAS_PADDING;
            max_x = canvas_half_width as i32;
        }

        if text_move.position.y < canvas_half_height as i32 {
            min_y = canvas_half_height as i32;
            max_y = canvas_size.height as i32 - CANVAS_PADDING;
        } else {
            min_y = CANVAS_PADDING;
            max_y = canvas_half_height as i32;
        }

        let next_target_x = get_next_text_target(min_x, max_x, validate_target_x);
        let next_target_y = get_next_text_target(min_y, max_y, validate_target_y);
        text_move.target = Point::new(next_target_x, next_target_y);
    }

    let position_x = ease(text_move.position.x, text_move.target.x, 0.002f32);
    let position_y = ease(text_move.position.y, text_move.target.y, 0.002f32);
    text_move.position = Point::new(position_x, position_y);
}

type ValidateTargetCallback = fn(target_point: i32, canvas_max: i32) -> bool;

fn get_next_text_target(canvas_min: i32,
                        canvas_max: i32,
                        callback: ValidateTargetCallback) -> i32 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(canvas_min .. canvas_max);

    loop {
        let throw = die.sample(&mut rng);
        if callback(throw, canvas_max) {
            return throw;
        }
    }
}

fn validate_target_x(target_point: i32, canvas_max: i32) -> bool {
    target_point > CANVAS_PADDING && target_point < canvas_max - TEXTURE_WIDTH
}

fn validate_target_y(target_point: i32, canvas_max: i32) -> bool {
    target_point > CANVAS_PADDING && target_point < canvas_max - TEXTURE_HEIGHT
}

fn ease(current: i32, target: i32, ease_time: f32) -> i32 {
    if current == target {
        return current;
    }
    let distance = target - current;
    let velocity = distance as f32 * ease_time;
    let mut position = current as f32 + velocity;
    if position as i32 == current {
        position += 1f32;
    }
    position as i32
}
