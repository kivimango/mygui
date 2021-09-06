use std::usize;
use orbclient::Color;
use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct TextComponent {
    pub font_family: String,
    pub font_size: usize,
    pub text: String,
    pub text_color: Color
}

impl Component for TextComponent {
    type Storage = VecStorage<Self>;
}

impl Default for TextComponent {
    fn default() -> Self {
        TextComponent {
            font_family: "Roboto-Medium".to_string(),
            font_size: 12,
            text: String::new(),
            text_color: Color::rgba(0, 0, 0, 255)
        }
    }
}