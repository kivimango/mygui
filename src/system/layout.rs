use specs::{Entity, World, WorldExt};
use vec_tree::VecTree;
use crate::{LayoutComponent, SizeComponent};

pub(crate) fn layout_system(world: &World) {
    let layouts = world.read_storage::<LayoutComponent>();
    let sizes = world.read_storage::<SizeComponent>();
    let tree = world.read_resource::<VecTree<Entity>>();

    if let Some(root_index) = tree.get_root_index() {
        for idx in tree.children(root_index) {
            if let Some(entity) = tree.get(idx) {
                if let Some(layout) = layouts.get(*entity) {
                    let desired_size = layout.object.measure(*entity, &layout.constraints, world);
                    println!("desired size for entity: {:?}:{:?}", desired_size, entity);
                    layout.object.arrange(*entity, &desired_size, world);
                }
            }
            
        }
    }
   
}