use specs::{Builder, Entity, World, WorldExt};
use vec_tree::{Index, VecTree};
use crate::{Constraints, DesiredSize, Layout, LayoutComponent, PositionComponent};

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
        println!("setting child in Center");
        self.child = Some(child);
        self
    }

    pub fn build(self, world: &mut World, parent_idx: Index) -> (Entity, Option<Index>) {
        println!("bulding Center");
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
        .build();

        if let Some(child) = self.child {
            let mut tree = world.write_resource::<VecTree<Entity>>();
            let layout_idx = tree.insert(child, parent_idx);
            return (widget, Some(layout_idx))
        }

        (widget, None)
    }
}

pub struct CenterLayout {}

impl Layout for CenterLayout {
    fn arrange(&self, widget: Entity, desired_size: &DesiredSize, world: &World) {
        let mut position = world.write_component::<PositionComponent>();
        if let Some(mut pos) = position.get_mut(widget) {
            let center_x = pos.x + ( desired_size.width / 2 );
            let center_y = pos.y + ( desired_size.height / 2 );
            pos.x = center_x;
            pos.y = center_y;
        }
    }

    fn measure(&self, _entity: Entity, constraints: &Constraints, _world: &World) -> DesiredSize {
        DesiredSize {
            dirty: false,
            height: constraints.max_height,
            width: constraints.max_width
        }
    }
}