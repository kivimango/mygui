# mygui

Experimental library for developing GUI applications for the Redox OS platform.

## Features

* Compact size (in terms of memory usage and binary size)
* Written in pure Rust (no wrappers over existing C/C++ libraries)
* Easy to use (no sorcery/black magic)

## Installation

The main branch is the latest stable branch that just compiles fine.
This project is not yet published on crates.io yet, so to include mygui as a dependency in your project, add this line to your *Cargo.toml* file:

```mygui = { git = "https://github.com/kivimango/mygui.git", branch = "main" }```

## Usage

Minimal starter template:

```rust
use mygui::{Application, Label, Window};

fn main() {
    Application::new()
        .name("mygui minimal example")
        .window(
            Window::new()
                .centered()
                .title("Minimal example")
                .ui(|world| {
                    Label::new("sample text".to_string())
                        .build(world)
                })
                .build(),
        )
        .build()
        .run();
}
```
