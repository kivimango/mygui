mod center;
mod rectangle;

use specs::{Entity, World};
use crate::{Constraints, DesiredSize, Layout};
pub use self::center::*;

pub struct NoopLayout {}

impl Layout for NoopLayout {
    fn arrange(&self, _widget: Entity, _desired_size: &DesiredSize, _world: &World) {
        
    }

    fn measure(&self, _entity: Entity, _constraints: &Constraints, _world: &World) -> DesiredSize {
        
        DesiredSize {dirty: false, width:0, height:0
        }
    }
}

impl NoopLayout {
    pub fn new() -> Self {
        NoopLayout {}
    }
}
