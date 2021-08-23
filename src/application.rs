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
