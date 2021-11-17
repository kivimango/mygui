use specs::{Entity, World};

pub struct Context<'w> {
    entity: Entity,
    world: &'w World
}

impl<'t, 'w> Context<'w> {
    pub fn new(entity: Entity, world: &'w World) -> Self {
        Context {
            entity,
            world
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Changes the current context into `new`, and returns the old one.
    /// Don't forget to change back to the `old` context after the changes were made.
    pub fn change_into(&mut self, new: Entity) -> Entity {
        let old_ctx = self.entity;
        self.entity = new;
        old_ctx
    }

}