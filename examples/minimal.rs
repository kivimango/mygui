use mygui::{Application, Label, Window};
use orbclient::Color;

fn main() {
    Application::new()
        .name("mygui minimal example")
        .window(
            Window::new()
                .x(500)
                .y(500)
                .width(155)
                .height(355)
                .title("Minimal example")
                .ui(|world| {
                    Label::new("sample text".to_string())
                        .background(Color::rgb(125, 100, 75))
                        .position(5, 5)
                        .size(110, 75)
                        .build(world)
                })
                .build(),
        )
        .build()
        .run();
}
