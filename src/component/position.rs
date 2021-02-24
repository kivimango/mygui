use specs::{Component, VecStorage};

#[derive(Debug, Default, Clone)]
pub struct PositionComponent {
    pub x: usize,
    pub y: usize
}

impl Component for PositionComponent {
    type Storage = VecStorage<Self>;
}