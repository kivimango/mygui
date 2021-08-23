use specs::{Component, VecStorage};

#[derive(Debug, Default, Clone)]
pub struct WindowComponent {
    pub id : u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Component for WindowComponent {
    type Storage = VecStorage<Self>;
}
