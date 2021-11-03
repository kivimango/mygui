use orbclient::WindowFlag;
use specs::{Entity, World};
use vec_tree::{Index};

type UiBuilder = dyn 'static + Fn(&mut World, Index) -> (Entity, Option<Index>);

pub struct Window {
    inner: orbclient::Window,
    ui: Option<Box<UiBuilder>>,
}

impl Window {
    pub fn new() -> WindowBuilder {
        WindowBuilder::new()
    }

    pub fn inner(&self) -> &orbclient::Window {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut orbclient::Window {
        &mut self.inner
    }

    pub fn ui(&self) -> &Option<Box<UiBuilder>> {
        &self.ui
    }
}

/// A builder for a Window widget.
/// It constructs an orbclient::Window object for an Application.
pub struct WindowBuilder {
    borderless: bool,
    height: u32,
    maximized: bool,
    resizeable: bool,
    title: String,
    transparent: bool,
    ui: Option<Box<UiBuilder>>,
    unclosable: bool,
    width: u32,
    x: i32,
    y: i32,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            borderless: false,
            height: 100,
            maximized: false,
            resizeable: true,
            title: String::new(),
            transparent: false,
            ui: None,
            unclosable: false,
            width: 100,
            x: 0,
            y: 0,
        }
    }

    /// Indicates that the window should be decorated with a border around it or not.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Starting height of the window in pixels.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Sets the window to the maximum available size.
    ///
    /// # Errors
    /// Calling this method has no effect when the applciation cannot determine the screen size.
    pub fn maximized(mut self) -> Self {
        match orbclient::get_display_size() {
            Ok(screen_size) => {
                self.width = screen_size.0;
                self.height = screen_size.1;
                self.maximized = true;
            }
            Err(msg) => {
                eprint!("Cannot maximize window: {}", msg);
            }
        }
        self
    }

    /// Sets the resizability of the window
    pub fn resizeable(mut self, resize: bool) -> Self {
        self.resizeable = resize;
        self
    }

    /// Sets the title of the window.
    pub fn title<S: AsRef<str>>(mut self, title: S) -> Self {
        self.title = title.as_ref().to_string();
        self
    }

    /// Sets the transparency of the window.
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Defines the UI of the window.
    pub fn ui<F: 'static + Fn(&mut World, Index) -> (Entity, Option<Index>)>(mut self, ui_builder: F) -> Self {
        self.ui = Some(Box::new(ui_builder));
        self
    }

    /// Sets the closability of the window.
    pub fn unclosable(mut self, unclosable: bool) -> Self {
        self.unclosable = unclosable;
        self
    }

    /// Starting width of the window in pixels.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Starting position of the top-left corner of the window on the horizontal axis of the screen.
    ///
    /// # Panics
    /// This method will panic when x parameter is less than 0.
    pub fn x(mut self, x: i32) -> Self {
        assert!(x > 0);
        self.x = x;
        self
    }

    /// Starting position of the top-left corner of the window on the vertical axis of the screen.
    ///
    /// # Panics
    /// This method will panic when y parameter is less than 0.
    pub fn y(mut self, y: i32) -> Self {
        assert!(y > 0);
        self.y = y;
        self
    }

    /// Constructs a Window object from the given properties.
    pub fn build(self) -> Window {
        let mut flags: Vec<WindowFlag> = Vec::new();
        if self.borderless {
            flags.push(WindowFlag::Borderless);
        }
        if self.unclosable {
            flags.push(WindowFlag::Unclosable);
        }
        if self.resizeable {
            flags.push(WindowFlag::Resizable);
        }
        if self.transparent {
            flags.push(WindowFlag::Transparent);
        }

        Window {
            inner: orbclient::Window::new_flags(
                self.x,
                self.y,
                self.width,
                self.height,
                &self.title,
                &flags,
            )
            .expect("Error creating window"),
            ui: self.ui,
        }
    }
}
