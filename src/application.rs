use std::{cell::RefCell, rc::Rc};

use minifb::{Key, Window};
use specs::{Entity, RunNow, World, WorldExt};
use crate::{EntityTree, Node, RenderingSystem, TextRenderSystem};

type UiBuilder = dyn 'static + Fn(&mut World) -> Entity;

pub struct Application {
    name: String,
    ui: Option<Box<UiBuilder>>,
    window: Window,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }

    pub fn run(&mut self) {
        let (width, height) = self.window.get_size();
        let buffer = Rc::new(RefCell::new(vec![0; width * height]));
        let mut world = World::new();

        if let Some(ui_builder) = &self.ui {
            let root_widget = ui_builder(&mut world);
            let root_node = Node::new(root_widget, None);
            let _entity_tree = EntityTree::new(root_node);
        }

        let mut text_system = TextRenderSystem{};
        let mut render_system = RenderingSystem::new(&buffer);

        self.window
            .limit_update_rate(Some(std::time::Duration::from_micros(8300)));

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            render_system.run_now(&mut world);
            text_system.run_now(&mut world);

            self.window
            .update_with_buffer(&buffer.borrow_mut(), 500, 500)
            .unwrap();
        }
    }
}

pub struct ApplicationBuilder {
    name: String,
    ui: Option<Box<UiBuilder>>,
    window: Option<Window>,
}

impl ApplicationBuilder {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {
            name: String::new(),
            ui: None,
            window: None,
        }
    }

    pub fn name(mut self, app_name: &str) -> Self {
        self.name = app_name.to_string();
        self
    }

    pub fn ui<F: 'static + Fn(&mut World) -> Entity>(mut self, ui_builder: F) -> Self {
        self.ui = Some(Box::new(ui_builder));
        self
    }

    pub fn window(mut self, window: Window) -> Self {
        self.window = Some(window);
        self
    }

    pub fn build(self) -> Application {
        Application {
            name: self.name,
            ui: self.ui,
            window: self.window.expect("You must define a Window on the Application."),
        }
    }
}

mod test {
    use crate::Application;
    use minifb::{Window, WindowOptions};

    #[test]
    fn test_buidler() {
        let app = Application::new()
            .name("test_app")
            .window(Window::new("test_app_window", 100, 100, WindowOptions::default()).unwrap())
            .build();

        assert_eq!(app.name, "test_app".to_string());
        assert_eq!(app.window.get_size(), (100, 100));
    }
}
