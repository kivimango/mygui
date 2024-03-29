use std::{cell::{Ref, RefCell}, rc::Rc};
use crate::{Constraints, EntityTree, LayoutComponent, RenderingSystem, Window, WindowComponent, WindowLayout, layout_system};
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
        //let root_id;
        // TODO: workaround the Rc<RefCell>> mess
        {
            let wb = self.window.borrow();
            root = entity_for_window(wb, &mut world);
            tree.add_node(root);
            tree.set_root(root);
            world.insert(tree);
        }

        if let Some(ui_builder) = self.window.borrow().ui() {
            let child_of_root = ui_builder(&mut world);
            let mut tree = world.write_resource::<EntityTree>();
            tree.add_node(child_of_root);
            tree.append_child(root, child_of_root);
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
    // TODO: add min and max size checking
    // updates the WindowComponent's size in the ECS world: PixMap needs it to be resize itself in the RenderingSystem
    {
        let mut layout_storage = world.write_component::<LayoutComponent>();
        if let Some(window_constraints) = layout_storage.get_mut(window_id) {
            window_constraints.constraints.max_width = event.width;
            window_constraints.constraints.max_height = event.height;
        } else {
            eprintln!("Cannot update window constraints after window resizing!");
        }
    }

    {
        let mut window_storage = world.write_component::<WindowComponent>();
        if let Some(window_component) = window_storage.get_mut(window_id) {
            window_component.width = event.width;
            window_component.height = event.height;
        } else {
            eprintln!("Cannot update window component's size after window resizing!");
        }
    }
}

fn entity_for_window(window: Ref<Window>, world: &mut World) -> Entity {
    world.register::<LayoutComponent>();
    world.register::<WindowComponent>();
    
    let layout = LayoutComponent {
        constraints: Constraints {
            min_width: window.min_width(),
            min_height: window.min_height(),
            max_width: window.max_width(),
            max_height: window.max_height(),
        },
        object: Box::new(WindowLayout{})
    };
    let window_component = WindowComponent {
        id: window.inner().id(),
        title: window.inner().title(),
        x: window.inner().x(),
        y: window.inner().y(),
        width: window.inner().width(),
        height: window.inner().height(),
    };
    world.create_entity()
        .with(window_component)
        .with(layout)
        .build()
}
