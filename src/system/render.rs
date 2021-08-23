use crate::{
    EntityTree, PositionComponent, RenderComponent, Shape, SizeComponent, Window, WindowComponent,
};
use orbclient::Renderer;
use specs::{Join, ReadStorage, System, World, WorldExt};
use std::{cell::RefCell, rc::Rc};
use tiny_skia::*;

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
        let mut brush = Paint::default();
        brush.set_color_rgba8(50, 127, 150, 200);
        brush.anti_alias = true;

        for (pos, render, size) in (&pos, &render, &size).join() {
            match render.shape {
                Shape::Circle(_radius) => {
                    todo!("todo: circle shape rendering");
                }
                Shape::Rectangle => {
                    println!(
                        "rendering at {} {} : w:{} h:{}",
                        pos.x, pos.y, size.width, size.height
                    );
                    let rect = Rect::from_xywh(
                        pos.x as f32,
                        pos.y as f32,
                        size.width as f32,
                        size.height as f32,
                    )
                    .unwrap();
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
