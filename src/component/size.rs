use specs::{Component, VecStorage};

#[derive(Copy, Clone, Debug, Default)]
pub struct SizeComponent {
    pub width: usize,
    pub height: usize
}

impl Component for SizeComponent {
    type Storage = VecStorage<Self>;
}