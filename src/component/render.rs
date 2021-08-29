use orbclient::Color;
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Circle(f64),
    Rectangle,
}

#[derive(Copy, Clone, Debug)]
pub struct RenderComponent {
    pub shape: Shape,
    pub background: Option<Color>
}

impl Component for RenderComponent {
    type Storage = VecStorage<Self>;
}
