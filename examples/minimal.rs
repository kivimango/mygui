use mygui::{Application, Label, Window};

fn main() {
    Application::new()
        .name("mygui minimal example")
        .window(
            Window::new()
                .x(100)
                .y(500)
                .width(155)
                .height(355)
                .title("Minimal example")
                .unclosable(true)
                .resizeable(false)
                .ui(|world| {
                    let label = Label::new("sample text".to_string()).build(world);
                    label.entity()
                })
                .build(),
        )
        .build()
        .run();
}
