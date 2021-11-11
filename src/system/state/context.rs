use specs::{Entity, World};
use crate::EntityTree;

pub struct Context<'t, 'w> {
    entity: Entity,
    tree: &'t mut EntityTree,
    world: &'w World
}

impl<'t, 'w> Context<'t, 'w> {
    pub fn new(entity: Entity, tree: &'t mut EntityTree, world: &'w World) -> Self {
        Context {
            entity,
            tree,
            world
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

}