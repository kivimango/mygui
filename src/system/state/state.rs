use crate::{Context, StateComponent};
use specs::{Join, World, WorldExt};

pub fn init_states(world: &mut World) {
    // TODO: init states when widgets dinamically added
    let mut states = world.write_storage::<StateComponent>();
    let entities = world.entities();

    for (state, entity) in (&mut states, &entities).join() {
        let mut context = Context::new(entity, &world);
        state.state_object.init(&mut context);
    }
}

pub fn state_system(world: &mut World) {
    let mut states = world.write_storage::<StateComponent>();
    let entities = world.entities();

    for (state, entity) in (&mut states, &entities).join() {
        let mut context = Context::new(entity, &world);
        state.state_object.update(&mut context);
    }
}