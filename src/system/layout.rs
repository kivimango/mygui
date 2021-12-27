use specs::{World, WorldExt};
use crate::{EntityTree, LayoutComponent};

pub fn layout_system(world: &World) {
    let layouts = world.read_storage::<LayoutComponent>();
    let tree = world.read_resource::<EntityTree>();

    if let Some(root) = tree.root() {
        let root_layout = layouts.get(root).expect("Root widget LayoutComponent not found !");
        let window_constraints = root_layout.constraints;
        let desired_child_size = root_layout.object.measure(root, &window_constraints, world);
        root_layout.object.arrange(root, &desired_child_size, world);
    }
}