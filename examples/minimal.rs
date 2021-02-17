use minifb::{Window, WindowOptions};
use mygui::Application;

fn main() {
    Application::new()
        .name("mygui minimal example")
        .window(Window::new("Minimal example", 500, 500, WindowOptions::default()).unwrap())
        .build()
        .run();
}
