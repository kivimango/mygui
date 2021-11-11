use crate::Context;
use orbclient::ButtonEvent;
use specs::{Component, HashMapStorage};

pub type OnClickHandler = dyn FnMut(&mut Context, ButtonEvent) + 'static;

pub struct MouseClickHandler {
    pub on_click_handler: Box<OnClickHandler>
}

impl Component for MouseClickHandler {
    type Storage = HashMapStorage<Self>;
}