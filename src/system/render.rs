use crate::{
    EntityTree, PositionComponent, RenderComponent, Shape, SizeComponent, Window, WindowComponent,
};
use orbclient::Renderer;
use specs::{Join, ReadStorage, System, World, WorldExt};
use std::{cell::RefCell, rc::Rc};
use tiny_skia::*;

/// Renders the visual representation of entities to the screen that has the following composition:
/// * PositionComponent
/// * RenderComponent
/// * SizeComponent
pub struct RenderingSystem<'w> {
    window: Rc<RefCell<Window>>,
    world: &'w World,
}

impl<'w> RenderingSystem<'w> {
    pub fn new(window: Rc<RefCell<Window>>, world: &World) -> RenderingSystem {
        RenderingSystem { window, world }
    }
}

impl<'s, 'w> System<'s> for RenderingSystem<'w> {
    type SystemData = (
        ReadStorage<'s, PositionComponent>,
        ReadStorage<'s, RenderComponent>,
        ReadStorage<'s, SizeComponent>,
    );

    fn run(&mut self, (pos, render, size): Self::SystemData) {
        let (width, height) = self.get_window_comp_size();
        let mut pixmap = Pixmap::new(width, height).unwrap();

        for (pos, render, size) in (&pos, &render, &size).join() {
            match render.shape {
                Shape::Circle(_radius) => {
                    todo!("todo: circle shape rendering");
                }
                Shape::Rectangle => {
                    let rect = Rect::from_xywh(
                        pos.x as f32,
                        pos.y as f32,
                        size.width as f32,
                        size.height as f32,
                    )
                    .unwrap();
                    let mut brush = Paint::default();
                    brush.anti_alias = true;
                    
                    if let Some(bg_color) = render.background {
                        let tinyskia_color = tiny_skia::Color::from_rgba8(bg_color.r(), bg_color.g(), bg_color.b(), bg_color.a());
                        brush.set_color(tinyskia_color);
                    } else {
                        // transparent background
                        brush.set_color(tiny_skia::Color::from_rgba8(0, 0, 0, 0,));
                    }
                    pixmap.fill_rect(rect, &brush, Transform::identity(), None);
                }
            }
        }

        self.swap_frame_buffer(pixmap.data_mut());
    }
}

impl<'w> RenderingSystem<'w> {
    fn get_window_comp_size(&self) -> (u32, u32) {
        let window = self.world.fetch::<EntityTree>().root().entity;
        let store = self.world.read_storage::<WindowComponent>();
        let window_component = store.get(window).unwrap();
        (window_component.width, window_component.height)
    }

    fn swap_frame_buffer(&mut self, bytes: &mut [u8]) {
        // frame buffer flipping code is borrowed from orbtk
        // https://github.com/redox-os/orbtk/blob/develop/orbtk_orbclient/src/orbclient/window.rs
        let len = bytes.len() / std::mem::size_of::<orbclient::Color>();
        let color_data = unsafe {
            std::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut orbclient::Color, len)
        };

        if color_data.len() == self.window.borrow().inner().data().len() {
            self.window
                .borrow_mut()
                .inner_mut()
                .data_mut()
                .clone_from_slice(color_data);
        }

        self.window.borrow_mut().inner_mut().sync();
    }
}
