use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub enum Shape {
    Circle(f64),
    Rectangle
}

#[derive(Debug, Clone)]
pub struct RenderComponent {
    pub shape: Shape
}

impl Component for RenderComponent {
    type Storage = VecStorage<Self>;
}