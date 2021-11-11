use mygui::{Application, Border, Button, Label, Window};
use orbclient::{Color};

fn main() {
    Application::new()
        .name("mygui minimal example")
        .window(
            Window::new()
                .centered()
                .title("Minimal example")
                .ui(|world| {
                    /*Label::new("sample text".to_string())
                        .background(Color::rgb(125, 100, 75))
                        .border(Border::new()
                            .color( orbclient::Color::rgba(12, 255, 12, 255))
                            .width(1.0)
                            .build()
                        )
                        .position(5, 5)
                        .size(110, 75)
                        .build(world)*/
                        Button::new()
                        .title("Button 1")
                        .on_click(move |_, _| {
                            println!("mouse click handler!");
                        })
                        .build(world)
                })
                .build(),
        )
        .build()
        .run();
}
