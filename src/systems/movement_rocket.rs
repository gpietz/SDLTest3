use crate::prelude::*;

#[system(for_each)]
#[read_component(Rocket)]
#[read_component(Point)]
pub fn movement_rocket(commands: &mut CommandBuffer,
                       entity: &Entity,
                       rocket: &Rocket,
                       position: &Point,
                       #[resource] canvas_size: &CanvasSize) {
    let rocket_speed = i8::from(rocket.0);
    let mut rocket_position = Point::from(*position);
    rocket_position = Point::new(rocket_position.x + (rocket_speed as i32), rocket_position.y());
    let space_ship_width = (SPACESHIP_WIDTH as i32) * 2;
    if rocket_speed < 0 && rocket_position.x() > -(space_ship_width) ||
        rocket_speed > 0 && rocket_position.x() < (canvas_size.width as i32 + space_ship_width) {
        commands.push((Rocket(rocket_speed), rocket_position));
    }
    commands.remove(*entity);
}
