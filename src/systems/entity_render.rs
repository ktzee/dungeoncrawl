use crate::prelude::*;

// NOTE: the "system" macro will append _system to the function name
#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // NOTE: We query for all entities that have both the Point and Render components
    <(&Point, &Render)>::query()
        // NOTE: we transform the query to an iterator. Requires the SubWorld. Returns a tuple
        .iter(ecs)
        // NOTE: deconstruct the tuple into names, use the coords from the camera
        // resource to calculate position
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph
            );
            }
        );
    // NOTE: we use 5000 for the ordering to leave some room for
    // other things to be drawn on the map or UI elements
    draw_batch.submit(5000).expect("Batch error");
}