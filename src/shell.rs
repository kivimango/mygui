use std::{cell::RefCell, rc::Rc};
use crate::{EntityTree, LayoutComponent, RenderingSystem, Window, WindowComponent, layout_system};
use orbclient::{Event, EventOption, Renderer, ResizeEvent};
use specs::{Builder, Entity, RunNow, World, WorldExt};

pub struct Shell {
    window: Rc<RefCell<Window>>,
}

impl Shell {
    pub fn new(window: Window) -> Self {
        Shell {
            window: Rc::new(RefCell::new(window)),
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
        let mut tree = EntityTree::new();
        let root;
        let root_id;
        // TODO: workaround the Rc<RefCell>> mess
        {
            let wb = self.window.borrow();
            root = entity_for_window(wb.inner(), &mut world);
            tree.add_node(root);
            root_id = tree.set_root(root);
            world.insert(tree);
        }

        world.register::<LayoutComponent>();

        if let Some(ui_builder) = self.window.borrow().ui() {
            let child_of_root = ui_builder(&mut world);
            let mut tree = world.write_resource::<EntityTree>();
            let child_id = tree.add_node(child_of_root);
            tree.append_child(root_id, child_id);
        }

        world.maintain();

        let mut render_system = RenderingSystem::new(self.window.clone(), &world);

        'event_loop: loop {
            for event in self.window.borrow_mut().inner_mut().events() {
                match event.to_option() {
                    EventOption::Quit(_) => {
                        break 'event_loop;
                    }
                    EventOption::Key(key_event) => println!("key event: {}", key_event.character),
                    EventOption::Mouse(_) => {}
                    EventOption::Resize(resize_event) => {
                        sync_window_size(root, &world, resize_event);
                    }
                    _ => {}
                }
            }

            layout_system(&world);
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
