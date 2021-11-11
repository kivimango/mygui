use crate::{Context, EntityTree, StateComponent};
use specs::{Join, World, WorldExt};

pub fn state_system(world: &mut World) {
    let mut states = world.write_storage::<StateComponent>();
    let entities = world.entities();
    let mut tree = world.write_resource::<EntityTree>();

    println!("in state");

    for (state, entity) in (&mut states, &entities).join() {
        let mut context = Context::new(entity, &mut tree, &world);
        state.state_object.update(&mut context);
    }
}