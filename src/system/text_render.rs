use specs::{Join, ReadStorage, System};
use crate::TextComponent;
pub struct TextRenderSystem {}

impl<'s> System<'s> for TextRenderSystem {
    type SystemData = ReadStorage<'s, TextComponent>;

    fn run(&mut self, texts: Self::SystemData) {
        for text in texts.join() {
            println!("{:?}", text);
        }
    }
}