use crate::prelude::*;

#[system]
#[read_component(Rocket)]
#[read_component(Point)]
pub fn render_rocket(ecs: &mut SubWorld) {
    let mut draw_batch = DrawBatch::new();
    <(&Rocket, &Point)>::query().iter(ecs).for_each(|(rocket, pos)| {
        let r = Rocket::from(*rocket);
        let p = Point::from(*pos);
        draw_batch.add_cmd(DrawImage {
            texture_id: TextureId::Spaceship,
            position: Some(p.clone()),
            rotate: 0f64,
            flip_horizontal: r.0 < 0,
            flip_vertical: false
        });
    });
    draw_batch.submit(200);
}
