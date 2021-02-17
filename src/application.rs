use minifb::{Key, Window};

pub struct Application {
    name: String,
    window: Window,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }

    pub fn run(&mut self) {
        let (width, height) = self.window.get_size();
        let mut buffer: Vec<u32> = vec![0; width * height];
        self.window
            .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
            .update_with_buffer(&buffer, 500, 500)
            .unwrap();
        }
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
