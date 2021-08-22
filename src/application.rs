use crate::{Shell, Window};

pub struct Application {
    name: String,
    shell: Shell,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }

    pub fn run(&mut self) {
        self.shell.run();

        /*let (width, height) = (self.window.width(), self.window.height());
        let buffer = Rc::new(RefCell::new(vec![0; width as usize * height as usize]));
        let mut world = World::new();

        if let Some(ui_builder) = &self.ui {
            let root_widget = ui_builder(&mut world);
            let root_node = Node::new(root_widget, None);
            let _entity_tree = EntityTree::new(root_node);
        }

        let mut text_system = TextRenderSystem{};
        let mut render_system = RenderingSystem::new(&buffer);*/

        //self.window
        //    .limit_update_rate(Some(std::time::Duration::from_micros(8300)));

        /*while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            render_system.run_now(&mut world);
            text_system.run_now(&mut world);

            self.window
            .update_with_buffer(&buffer.borrow_mut(), 500, 500)
            .unwrap();
        }*/
    }
}

pub struct ApplicationBuilder {
    name: String,
    window: Option<Window>,
}

impl ApplicationBuilder {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {
            name: String::new(),
            window: None,
        }
    }

    pub fn name(mut self, app_name: &str) -> Self {
        self.name = app_name.to_string();
        self
    }

    pub fn window(mut self, window: Window) -> Self {
        self.window = Some(window);
        self
    }

    pub fn build(self) -> Application {
        Application {
            name: self.name,
            shell: Shell::new(
                self.window
                    .expect("You must define at least one window on the application."),
            ),
        }
    }
}
