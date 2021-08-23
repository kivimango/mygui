use std::{borrow::BorrowMut, rc::Rc};

use specs::{Builder, Entity, World, WorldExt};
use crate::{Node, tree::{self, EntityTree}};

pub struct HBox<'w> {
    entity: Entity,
    layout: LayoutComponent,
    world: Rc<&'w mut World>
}

impl HBox<'_> {
    pub fn new() -> HBoxBuilder {
        HBoxBuilder::new()
    }
}

pub struct HBoxBuilder {
    children: Vec<Entity>,
    layout: LayoutComponent,
    spacing: usize
}

impl HBoxBuilder {
    pub fn new() -> HBoxBuilder {
        HBoxBuilder {
            children: vec![],
            layout: LayoutComponent {
                layout: Box::new(HBoxLayout::new())
            },
            spacing: 0
        }
    }

    pub fn child(mut self, child: Entity) -> Self {
        self.children.push(child);
        self
    }

    pub fn spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn build(self, world: &mut World) -> HBox {
        let entity = world.create_entity().build();
        {
            let mut tree = world.write_resource::<EntityTree>();

            for child in self.children {
                let node = Node::new(child, Some(entity));
                tree.borrow_mut().add_child(node);
            }
        }

        HBox {
            entity,
            layout: self.layout,
            world: Rc::from(world)
        }
    }
}

pub trait Layout {
    fn measure(&self);
    fn arrange(&self);
}

pub struct LayoutComponent {
    layout: Box<dyn Layout>
}

pub struct HBoxLayout {

}

impl HBoxLayout {
    pub fn new() -> HBoxLayout {
        HBoxLayout {}
    }
}

impl Layout for HBoxLayout {
    fn measure(&self) {
        
    }

    fn arrange(&self) {
        
    }
}