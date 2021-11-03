use crate::{PositionComponent, RenderComponent, Shape, SizeComponent, TextComponent, Window, WindowComponent};
use orbclient::Renderer;
use rusttype::{OutlineBuilder, Point, PositionedGlyph, Scale, point};
use specs::{Entity, Join, ReadStorage, System, World, WorldExt};
use vec_tree::VecTree;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tiny_skia::*;

pub const DEFAULT_FONT_FAMILY: &'static[u8] = include_bytes!("../../assets/fonts/Roboto-Medium.ttf");

struct GlyphTracer {
    path_builder: PathBuilder,
    position: Point<f32>
}

impl GlyphTracer {
    #[inline(always)]
    fn map_point(&self, x: f32, y: f32) -> (f32, f32) {
        (self.position.x + x, self.position.y + y)
    }
}

impl OutlineBuilder for GlyphTracer {
    fn move_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.map_point(x, y);
        self.path_builder.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.map_point(x, y);
        self.path_builder.line_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (x, y) = self.map_point(x, y);
        let (x1, y1) = self.map_point(x1, y1);
        self.path_builder.quad_to(x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (x, y) = self.map_point(x, y);
        let (x1, y1) = self.map_point(x1, y1);
        let (x2, y2) = self.map_point(x2, y2);
        self.path_builder.cubic_to(x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        self.path_builder.close();
    }
}

/// Renders the visual representation of entities to the screen that has the following composition:
/// * PositionComponent
/// * RenderComponent
/// * SizeComponent
pub struct RenderingSystem<'w> {
    window: Rc<RefCell<Window>>,
    world: &'w World,
    fonts: HashMap<String, rusttype::Font<'static>>
}

impl<'w> RenderingSystem<'w> {
    pub fn new(window: Rc<RefCell<Window>>, world: &World) -> RenderingSystem {
        let mut fonts: HashMap<String, rusttype::Font> = HashMap::new();
        let result = rusttype::Font::try_from_bytes(DEFAULT_FONT_FAMILY);
        if let Some(f) = result {
            fonts.insert("Roboto-Medium".to_string(), f);
        }
        RenderingSystem { window, world, fonts }
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

                    // draw border around widget rectangle
                    // TODO: implement rounded border
                    if let Some(border) = render.border {
                        let path = PathBuilder::from_rect(rect);
                        let tinyskia_color = tiny_skia::Color::from_rgba8(border.color.r(), border.color.g(), border.color.b(), border.color.a());
                        brush.set_color(tinyskia_color);
                        let mut stroke = Stroke::default();
                        stroke.width = border.top;
                        pixmap.stroke_path(&path, &brush, &stroke, Transform::identity(), None);
                    }
                }
            }
        }
        
        self.render_text(&mut pixmap);

        self.swap_frame_buffer(pixmap.data_mut());
    }
}

impl<'w> RenderingSystem<'w> {
    fn get_window_comp_size(&self) -> (u32, u32) {
        let tree = self.world.fetch::<VecTree<Entity>>();
        let window_idx = tree.get_root_index()
            .expect("Root widget not found in the EntityTree! Please set a Window widget as a root of the tree.");
        let window = tree.get(window_idx)
            .expect("Root widget not found in the EntityTree! Please set a Window widget as a root of the tree.");
        let store = self.world.read_storage::<WindowComponent>();
        let window_component = store.get(*window).unwrap();
        (window_component.width, window_component.height)
    }

    fn render_text(&self, pixmap: &mut Pixmap) {
        let positions = self.world.read_storage::<PositionComponent>();
        let texts = self.world.read_storage::<TextComponent>();

        for (position, text_comp) in (&positions, &texts).join() {
            // text rendering code is based on orbtk
            // https://github.com/redox-os/orbtk/blob/develop/orbtk_tinyskia/src/tinyskia/font.rs
            if let Some(font) = self.fonts.get(&text_comp.font_family) {
                let scale = Scale::uniform(text_comp.font_size as f32);

                // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
                // We don't want to clip the text, so we shift it down with an offset when laying it out.
                // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
                // the font. That's enough to guarantee that there's no clipping.
                let v_metrics = font.v_metrics(scale);
                let offset = point(0.0, v_metrics.ascent);
                let glyphs: Vec<PositionedGlyph> = font.layout(text_comp.text.as_str(), scale, offset).collect();
                let mut glyph_tracer = GlyphTracer {
                    path_builder: PathBuilder::new(),
                    position: point(0.0, 0.0)
                };
    
                for g in glyphs.iter() {
                    let mut gpos = match g.pixel_bounding_box() {
                        Some(bbox) => rusttype::point(bbox.min.x as f32, bbox.min.y as f32),
                        None => {
                            continue;
                        }
                    };
                    gpos.x += position.x as f32;
                    gpos.y += position.y as f32;
                    glyph_tracer.position = gpos;
                    g.build_outline(&mut glyph_tracer);
                }
    
                let mut brush = Paint::default();
                brush.anti_alias = true;
                let text_color = tiny_skia::Color::from_rgba8(text_comp.text_color.r(), text_comp.text_color.g(), text_comp.text_color.b(), text_comp.text_color.a());
                brush.set_color(text_color);
    
                if let Some(path) = glyph_tracer.path_builder.finish() {
                    pixmap.fill_path(&path, &brush, FillRule::Winding, Transform::identity(), None);
                }   
            }
            // else {
            // TODO: load fonts lazily, cross-platform font loader ???
            //}
        }
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
