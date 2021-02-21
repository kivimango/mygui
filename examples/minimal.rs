use minifb::{Window, WindowOptions};
use mygui::{Application, Label};

fn main() {
    Application::new()
        .name("mygui minimal example")
        .window(Window::new("Minimal example", 500, 500, WindowOptions::default()).unwrap())
        .ui(|ctx| {
            let label = Label::new("sample text".to_string())
            .build(ctx);
            label.entity()
        })
        .build()
        .run();
}
