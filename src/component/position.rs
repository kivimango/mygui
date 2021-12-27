use specs::{Component, VecStorage};

#[derive(Copy, Clone, Debug, Default)]
pub struct PositionComponent {
    pub x: u32,
    pub y: u32,
}

impl Component for PositionComponent {
    type Storage = VecStorage<Self>;
}
