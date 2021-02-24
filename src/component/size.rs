use specs::{Component, VecStorage};

#[derive(Debug, Default, Clone)]
pub struct SizeComponent {
    pub width: usize,
    pub height: usize
}

impl Component for SizeComponent {
    type Storage = VecStorage<Self>;
}