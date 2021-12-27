use specs::{Entity, World, WorldExt};
use crate::{Layout, Constraints, DesiredSize, EntityTree, LayoutComponent, WindowComponent};

pub struct WindowLayout {}

impl Layout for WindowLayout {
    fn arrange(&self, root: Entity, desired_size: &DesiredSize, world: &World) {
        let tree = world.read_resource::<EntityTree>();
        let layouts = world.read_storage::<LayoutComponent>();

        if let Some(child) = tree.child_of(root) {
            if let Some(child_layout) = layouts.get(child) {
                child_layout.object.arrange(child, desired_size, world)
            }
        }
    }

    fn measure(&self, root: Entity, root_constraints: &Constraints, world: &World) -> DesiredSize {
        let tree = world.read_resource::<EntityTree>();
        let layouts = world.read_storage::<LayoutComponent>();
        let windows = world.read_component::<WindowComponent>();

        if let Some(child) = tree.child_of(root) {
            if let Some(child_layout) = layouts.get(child) {
                let window_comp = windows.get(root).expect("WindowLayout: No WindowComponent found for root!");
                // mixing minimum constraints with current window size as maximum constraints
                let constraints = Constraints {
                    min_height: root_constraints.min_height,
                    min_width: root_constraints.min_width,
                    max_height: window_comp.height,
                    max_width: window_comp.width
                };
                child_layout.object.measure(child, &constraints, world)
            } else {
                let root_layout = layouts.get(root).unwrap();
                return DesiredSize {
                    dirty: false,
                    width: root_layout.constraints.max_width,
                    height: root_layout.constraints.max_height
                }
            }
        } else {
            // An edges-case: the root has no child, just return the actual size of the window
            let root_layout = layouts.get(root).unwrap();
            return DesiredSize {
                dirty: false,
                width: root_layout.constraints.max_width,
                height: root_layout.constraints.max_height
            }
        }
    }
}