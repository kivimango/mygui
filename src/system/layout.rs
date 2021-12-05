use specs::{World, WorldExt};
use crate::{Constraints, EntityTree, LayoutComponent, SizeComponent};

pub(crate) fn layout_system(world: &World) {
    let layouts = world.read_storage::<LayoutComponent>();
    let sizes = world.read_storage::<SizeComponent>();
    let tree = world.read_resource::<EntityTree>();

    if let Some(_root) = tree.root() {
        for node in tree.children() {
            let entity = node.get();
            if let Some(layout) = layouts.get(*entity) {
                let desired_size = layout.object.measure(*entity, &layout.constraints, world);
                println!("desired size for entity: {:?}:{:?}", desired_size, entity);
                layout.object.arrange(*entity, &desired_size, world);
            }
        }
    }
}