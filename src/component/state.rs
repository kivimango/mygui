use specs::{Component, VecStorage};
use crate::State;

pub struct StateComponent {
    pub enabled: bool,
    pub state_object: Box<dyn State>
}

impl Component for StateComponent {
    type Storage = VecStorage<Self>;
}