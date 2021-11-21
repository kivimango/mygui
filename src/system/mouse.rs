use orbclient::{ButtonEvent, MouseEvent};
use specs::{Join, World, WorldExt};
use crate::{Context, MouseClickHandler, PositionComponent, SizeComponent};

/// Handles mouse clicks on widgets.
/// Widgets must be composited with the following components to able to handle mouse clicks:
/// * PositionComponent
/// * SizeComponent
/// * MouseClickHandler
///
/// If the mouse pointer is hovered on a widget, MouseClickHandler will be called.
pub fn mouse_system(world: &World, mouse_pos: MouseEvent, mouse_button_event: ButtonEvent) {
    let mut clickables = world.write_component::<MouseClickHandler>();
    let positions = world.read_component::<PositionComponent>();
    let sizes = world.read_component::<SizeComponent>();
    let entities = world.entities();

    for (entity, pos, size, handler) in (&entities, &positions, &sizes, &mut clickables).join() {
        // TODO: edges case: if widgets are stacked on top of each other, use a z-index
        if intersects(pos, size, &mouse_pos) {
            let mut context = Context::new(entity, world);
            (handler.on_click_handler)(&mut context, mouse_button_event);
        }
    }
}

fn intersects(position: &PositionComponent, size: &SizeComponent, mouse_pos: &MouseEvent) -> bool {
    // TODO: fix converting, make position and size x,y type to i32 ?
    use std::convert::TryInto;
    let pos_x = position.x.try_into().unwrap();
    let pos_y = position.y.try_into().unwrap();

    if (mouse_pos.x > pos_x && mouse_pos.x < pos_x + size.width as i32) && (mouse_pos.y > pos_y && mouse_pos.y < pos_y + size.height as i32) {
        return true;
    }
    false
}