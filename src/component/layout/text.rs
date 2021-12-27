use specs::{WorldExt, Entity, World};
use crate::{Layout, DesiredSize, Constraints, SizeComponent};

pub struct TextLayout {}

impl Layout for TextLayout {
    fn arrange(&self, _widget: Entity, _desired_size: &DesiredSize, _world: &World) {
        
    }

    fn measure(&self, entity: Entity, _constraints: &Constraints, world: &World) -> DesiredSize {
        let sizes = world.read_storage::<SizeComponent>();

        if let Some(size) = sizes.get(entity) {
            DesiredSize {
                dirty: false,
                width: size.width as u32,
                height: size.height as u32
            }
        } else {
            DesiredSize {
                dirty: false,
                width: 0,
                height: 0
            }
        }
    }
}