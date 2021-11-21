use std::{cell::RefCell, rc::Rc};

use crate::{EntityTree, Node, RenderingSystem, Window, WindowComponent, init_states, state_system, mouse_system};
use orbclient::{Event, EventOption, MouseEvent, Renderer, ResizeEvent};
use specs::{Builder, Entity, RunNow, World, WorldExt};

pub struct Shell {
    window: Rc<RefCell<Window>>,
    mouse_pos: MouseEvent
}

impl Shell {
    pub fn new(window: Window) -> Self {
        Shell {
            window: Rc::new(RefCell::new(window)),
            mouse_pos: MouseEvent {x: 0, y:0}
        }
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
        let root;
        // TODO: workaround the Rc<RefCell>> mess
        {
            let wb = self.window.borrow();
            root = entity_for_window(wb.inner(), &mut world);
        }

        if let Some(ui_builder) = self.window.borrow().ui() {
            let ui = ui_builder(&mut world);
            let mut root_node = Node::new(root, None);
            let ui_node = Node::new(ui, Some(root));
            root_node.append_child(&root, ui_node);

            let entity_tree = EntityTree::new(root_node);
            world.insert(entity_tree);
        }

        world.maintain();

        init_states(&mut world);
        state_system(&mut world);

        let mut render_system = RenderingSystem::new(self.window.clone(), &world);

        'event_loop: loop {
            for event in self.window.borrow_mut().inner_mut().events() {
                match event.to_option() {
                    EventOption::Button(button_event) => {
                        println!("mouse event received");
                        mouse_system(&world, self.mouse_pos, button_event);
                    }
                    EventOption::Quit(_) => {
                        break 'event_loop;
                    }
                    EventOption::Key(key_event) => println!("key event: {}", key_event.character),
                    EventOption::Mouse(mouse_pos) => {
                        self.mouse_pos = mouse_pos;
                    }
                    EventOption::Resize(resize_event) => {
                        sync_window_size(root, &world, resize_event);
                    }
                    _ => {}
                }
            }

            render_system.run_now(&world);
        }
    }
}

fn sync_window_size(window_id: Entity, world: &World, event: ResizeEvent) {
    // updates the WindowComponent's size in the ECS world: PixMap needs it to be resize itself in the RenderingSystem
    let mut storage = world.write_component::<WindowComponent>();
    if let Some(window_component) = storage.get_mut(window_id) {
        window_component.width = event.width;
        window_component.height = event.height;
    }
}

fn entity_for_window(window: &orbclient::Window, world: &mut World) -> Entity {
    world.register::<WindowComponent>();
    let window_component = WindowComponent {
        id: window.id(),
        title: window.title(),
        x: window.x(),
        y: window.y(),
        width: window.width(),
        height: window.height(),
    };
    world.create_entity().with(window_component).build()
}
