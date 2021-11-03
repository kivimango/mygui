use specs::{Component, Entity, VecStorage, World};
use crate::{Constraints};

pub trait Layout {
    fn arrange(&self, widget: Entity, desired_size: &DesiredSize, world: &World);
    fn measure(&self, entity: Entity, constraints: &Constraints, world: &World) -> DesiredSize;
}

pub struct LayoutComponent {
    pub constraints: Constraints,
    pub object: Box<dyn Layout>
}

impl Component for LayoutComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct DesiredSize {
    pub dirty: bool,
    pub width: usize,
    pub height: usize
}