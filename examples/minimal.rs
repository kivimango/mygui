use mygui::{Application, Border, Center, Label, Window};
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
                    Center::new()
                        .child(
                            Label::new("sample text".to_string())
                                .background(Color::rgb(125, 100, 75))
                                .border(
                                    Border::new()
                                    .color(Color::rgba(12, 255, 12, 255))
                                    .width(1.0)
                                    .build()
                                )
                            .position(5, 5)
                            .size(110, 75)
                            .build(world)
                        )
                        .build(world)
                })
                .build(),
        )
        .build()
        .run();
}
