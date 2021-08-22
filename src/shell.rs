use crate::{EntityTree, Node, Window};
use orbclient::{Event, EventOption};
use specs::{World, WorldExt};

pub struct Shell {
    window: Window,
}

impl Shell {
    pub fn new(window: Window) -> Self {
        Shell { window }
    }

    /// Handles events coming from orbclient.
    /// Each event is consumed by its respective system.
    pub fn on_event(event: Event) {
        match event.to_option() {
            // TODO: implement clipboard & drag and drop operations in the future
            EventOption::Button(_) => {}
            EventOption::Clipboard(_) => {}
            EventOption::ClipboardUpdate(_) => {}
            EventOption::Drop(_) => {}
            EventOption::Focus(_) => {}
            EventOption::Hover(_) => {}
            EventOption::Key(_) => {}
            EventOption::Mouse(_) => {}
            EventOption::MouseRelative(_) => {}
            EventOption::Move(_) => {}
            EventOption::None => {}
            EventOption::Quit(_) => {}
            EventOption::Resize(_) => {}
            EventOption::Screen(_) => {}
            EventOption::Scroll(_) => {}
            EventOption::TextInput(_) => {}
            EventOption::Unknown(_) => {}
        }
    }

    pub fn run(&mut self) {
        let mut world = World::new();

        if let Some(ui_builder) = self.window.ui() {
            let root_widget = ui_builder(&mut world);
            let root_node = Node::new(root_widget, None);
            let _entity_tree = EntityTree::new(root_node);
        }

        'event_loop: loop {
            for event in self.window.inner().events() {
                match event.to_option() {
                    EventOption::Quit(_) => {
                        break 'event_loop;
                    }
                    EventOption::Mouse(_) => {}
                    _ => {}
                }
            }
        }
    }
}
