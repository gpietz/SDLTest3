use crate::prelude::*;

#[system]
#[read_component(BouncingText)]
#[write_component(BouncingTextMove)]
pub fn render_text(ecs: &mut SubWorld) {
    let mut draw_batch = DrawBatch::new();
    <(&BouncingText, &BouncingTextMove)>::query().iter(ecs).for_each(|(text, text_move)| {
        let bouncing_text_move = *text_move;
        draw_batch.add_cmd(DrawText {
            text: text.0.clone(),
            position: bouncing_text_move.position,
        });
    });

    draw_batch.submit(1000);
}
