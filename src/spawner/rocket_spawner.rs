pub use crate::prelude::*;

use std::time::{SystemTime, Duration};
use std::ops::Sub;
use rand::distributions::{Distribution, Uniform};
use sdl2::render::WindowCanvas;

const ROCKET_MAX_SPEED : i8 = 5;
const SPAWN_DELAY : u128 = 500;
const SPAWN_PADDING : u32 = SPACESHIP_HEIGHT;
const SPAWN_MIN_DISTANCE: u32 = 40;
const SPAWN_CYCLE_INCREASE_DELAY : u8 = 2;

pub struct RocketSpawner {
    last_rocket_spawned: SystemTime,
    last_spawn_cycles_increased: SystemTime,
    last_spawn_offset: u32,
    last_spawn_speed: i8,
    spawn_cycles: usize,
    rockets_spawned: u32,
}

impl RocketSpawner {
    pub fn new() -> Self {
        Self {
            last_rocket_spawned: SystemTime::now().sub(Duration::new(SPAWN_DELAY as u64, 0)),
            last_spawn_cycles_increased: SystemTime::now(),
            last_spawn_offset: 0,
            last_spawn_speed: 0,
            rockets_spawned: 0,
            spawn_cycles: 1
        }
    }

    pub fn spawn(&mut self, ecs: &mut World, canvas: &mut WindowCanvas) -> bool {
        if self.rockets_spawned > 0 && self.last_rocket_spawned
            .elapsed()
            .unwrap()
            .as_millis() <= SPAWN_DELAY {
            return false;
        }

        if self.last_spawn_cycles_increased
            .elapsed()
            .unwrap()
            .as_secs() >= SPAWN_CYCLE_INCREASE_DELAY as u64 {
            self.last_spawn_cycles_increased = SystemTime::now();
            self.spawn_cycles += 1;
        }

        let (canvas_width, ..) = canvas.output_size().unwrap();
        let spaceship_width = (SPACESHIP_WIDTH as i32) * 2;

        for _  in 0 ..= self.spawn_cycles {
            let rocket_speed = self.get_rocket_speed();
            let horizontal_offset = if rocket_speed > 0 {
                -spaceship_width
            } else {
                canvas_width as i32 + spaceship_width
            };
            let vertical_offset = self.get_vertical_offset(canvas);
            ecs.push((Rocket(rocket_speed), Point::new(horizontal_offset, vertical_offset as i32)));

            self.last_rocket_spawned = SystemTime::now();
            self.last_spawn_offset = vertical_offset;
            self.last_spawn_speed = rocket_speed;
            self.rockets_spawned += 1;
        }

        true
    }

    fn get_vertical_offset(&self, canvas: &mut WindowCanvas) -> u32 {
        let (.., height) = canvas.output_size().unwrap();
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1 + SPAWN_PADDING .. height - (SPAWN_PADDING * 2));
        let half_height = height/2;

        loop {
            let throw = die.sample(&mut rng);
            let distance = (self.last_spawn_offset as i32 - throw as i32).abs() as u32;
            let same_area = self.last_spawn_offset < half_height && throw < half_height;
            if (self.last_spawn_offset == 0 || distance > SPAWN_MIN_DISTANCE) &&
                throw > SPAWN_PADDING &&
                throw < (height - SPAWN_PADDING) &&
                !same_area {
                return throw
            }
        }
    }

    fn get_rocket_speed(&self) -> i8 {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(-(ROCKET_MAX_SPEED)  ..= ROCKET_MAX_SPEED);
        loop {
            let throw = die.sample(&mut rng);
            if throw != 0 &&
                !(self.last_spawn_speed < 0 && throw < 0) &&
                !(self.last_spawn_speed > 0 && throw > 0) {
                return throw
            }
        }
    }
}
