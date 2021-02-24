
use std::{cell::{RefCell}, rc::Rc};
use specs::{Join, ReadStorage, System};
use tiny_skia::*;
use crate::{PositionComponent, RenderComponent, Shape, SizeComponent};

pub struct RenderingSystem {
    frame_buffer: Rc<RefCell<Vec<u32>>>
}

impl RenderingSystem {
    pub fn new(fb: &Rc<RefCell<Vec<u32>>>) -> RenderingSystem {
        RenderingSystem {
            frame_buffer: fb.clone()
        }
    }
}

impl<'s> System<'s> for RenderingSystem {
    type SystemData = (ReadStorage<'s, PositionComponent>, ReadStorage<'s, RenderComponent>, ReadStorage<'s, SizeComponent>);

    fn run(&mut self, (pos, render, size): Self::SystemData) {
        // TODO: implement a 2DRenderingContext maybe ?
        let bg_color = Color::from_rgba8(246, 245, 244, 100);

        let mut paint = Paint::default();
        paint.set_color(bg_color);
        paint.anti_alias = true;

        let mut p = Pixmap::new(500, 500).unwrap();
        let pixmap = PixmapMut::from(p.as_mut());
        let mut canvas = Canvas::from(pixmap);

        let background = Rect::from_xywh(0.0, 0.0, 500.0, 500.0).unwrap();
        canvas.fill_rect(background, &paint);

        for (pos, render, size) in (&pos, &render, &size).join() {
            match render.shape {
                Shape::Circle(_radius) => {
                    println!("todo: circle shape rendering");
                }
                Shape::Rectangle => {
                    let rect = Rect::from_xywh(pos.x as f32, pos.y as f32, size.width as f32, size.height as f32).unwrap();
                    paint.set_color_rgba8(50,127, 150, 200);
                    canvas.fill_rect(rect, &paint);
                }
            }

            let data: Vec<u32> = canvas.pixmap().pixels_mut().iter()
                .map(|p| {
                    p.get()
                })
                .collect();
            self.frame_buffer.borrow_mut().copy_from_slice(&data);
        }        
    }
}

