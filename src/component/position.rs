use specs::{Component, VecStorage};

#[derive(Copy, Clone, Debug, Default)]
pub struct PositionComponent {
    pub x: usize,
    pub y: usize
}

impl Component for PositionComponent {
    type Storage = VecStorage<Self>;
}
