use specs::{Component, VecStorage};

#[derive(Debug, Default, Clone)]
pub struct TextComponent {
    pub text: String,
}

impl Component for TextComponent {
    type Storage = VecStorage<Self>;
}

impl From<String> for TextComponent {
    fn from(text: String) -> Self {
        TextComponent { text }
    }
}
