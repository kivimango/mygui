use specs::{Builder, Entity, World, WorldExt};
use crate::{Constraints, DesiredSize, EntityTree, Layout, LayoutComponent, PositionComponent, SizeComponent};

/// A layout widget that centers its child wihtin itself.
pub struct Center {}

pub struct CenterBuilder {
    child: Option<Entity>
}

impl Center {
    pub fn new() -> CenterBuilder {
        CenterBuilder::new()
    }
}

impl CenterBuilder {
    fn new() -> Self {
        CenterBuilder {
            child: None,
        }
    }

    pub fn child(mut self, child: Entity) -> Self {
        self.child = Some(child);
        self
    }

    pub fn build(self, world: &mut World) -> Entity {
        world.register::<LayoutComponent>();
        
        let layout = LayoutComponent {
            constraints: Constraints {
                min_height: 0,
                min_width: 0,
                max_width: 0,
                max_height: 0
            },
            object: Box::new(CenterLayout {}),
        };

        let widget = world.create_entity()
        .with(layout)
        .with(PositionComponent {
            x:0, y:0
        })
        .build();

        if let Some(child) = self.child {
            let mut tree = world.write_resource::<EntityTree>();
            tree.add_node(child);
            tree.add_node(widget);
            tree.append_child(widget, child);
        }

        widget
    }
}

pub struct CenterLayout {}

impl Layout for CenterLayout {
    fn arrange(&self, widget: Entity, desired_size: &DesiredSize, world: &World) {
        let tree = world.read_resource::<EntityTree>();
        let sizes = world.read_storage::<SizeComponent>();

        if let Some(child) = tree.child_of(widget) {
            let mut position = world.write_component::<PositionComponent>();
            if let Some(mut pos) = position.get_mut(child) {
                if let Some(child_size) = sizes.get(child) {
                    pos.x = (desired_size.width - child_size.width as u32) / 2;
                    pos.y = (desired_size.height - child_size.height as u32) / 2;
                }
                println!("desired size: {:?}", desired_size);
                println!("new coords: {}-{}", pos.x, pos.y);
            }
        }
    }

    fn measure(&self, _entity: Entity, constraints: &Constraints, _world: &World) -> DesiredSize {
        DesiredSize {
            dirty: false,
            width: constraints.max_width,
            height: constraints.max_height
        }
    }
}