use orbclient::Color;
use specs::{Component, VecStorage};
use crate::Border;

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Circle(f64),
    Rectangle,
}

#[derive(Copy, Clone, Debug)]
pub struct RenderComponent {
    pub shape: Shape,
    pub background: Option<Color>,
    pub border: Option<Border>
}

impl Component for RenderComponent {
    type Storage = VecStorage<Self>;
}
