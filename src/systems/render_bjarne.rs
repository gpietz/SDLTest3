use crate::prelude::*;

/**
 * Draws the bjarne on the screen. ;)
 */
#[system]
#[read_component(Bjarne)]
#[write_component(Image)]
pub fn render_bjarne(ecs: &mut SubWorld) {
    let mut draw_batch = DrawBatch::new();
    let bjarne = <&Image>::query().filter(component::<Bjarne>())
        .iter(ecs)
        .nth(0)
        .unwrap();
    draw_batch.add_cmd(bjarne.to_draw_command());
    draw_batch.submit(100);
}
