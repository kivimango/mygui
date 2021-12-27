use orbclient::WindowFlag;
use specs::{Entity, World};

type UiBuilder = dyn 'static + Fn(&mut World) -> Entity;

pub struct Window {
    inner: orbclient::Window,
    max_height: u32,
    max_width: u32,
    min_height: u32,
    min_width: u32,
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

    pub fn min_height(&self) -> u32 {
        self.min_height
    }

    pub fn min_width(&self) -> u32 {
        self.min_width
    }

    pub fn max_height(&self) -> u32 {
        self.max_height
    }

    pub fn max_width(&self) -> u32 {
        self.max_width
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
    max_height: u32,
    max_width: u32,
    min_height: u32,
    min_width: u32,
    maximized: bool,
    max_height_set: bool,
    max_width_set: bool,
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
            max_height: 65535,
            max_width: 65535,
            min_height: 0,
            min_width: 0,
            maximized: false,
            max_height_set: false,
            max_width_set: false,
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

    /// Resizes and places the center of the window to the center of the screen.
    /// 
    /// Notes:
    /// * Calling this method has no effect when the application cannot determine the screen size.
    /// * Calling [width()](#method.width), [height()](#method.height), [x()](#method.x), [y()](#method.y)
    /// or [maximized()](#method.maximized) after this method will overwrite this method's behavior.
    pub fn centered(mut self) -> Self {
        use std::convert::TryFrom;

        match orbclient::get_display_size() {
            Ok(screen_size) => {
                if self.width > screen_size.0 || self.height > screen_size.1 {
                    self.width = screen_size.0;
                    self.height = screen_size.1;
                }

                self.width = screen_size.0 / 2;
                self.height = screen_size.1 / 2;
                self.x = i32::try_from((screen_size.0 - self.width) / 2).unwrap_or(0);
                self.y = i32::try_from((screen_size.1 - self.height) / 2).unwrap_or(0);
            }
            Err(msg) => {
                eprintln!("Cannot determine screen size: {}", msg);
            }
        }
        self
    }

    /// Starting height of the window in pixels.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Sets the amount of maximum height the window should be resized.
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self.max_height_set = true;
        self
    }

    /// Sets the amount of maximum width the window should be resized.
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self.max_width_set = true;
        self
    }

    /// Sets the least amount of minimum height the window should be resized.
    pub fn min_height(mut self, min_height: u32) -> Self {
        self.min_height = min_height;
        self
    }

    /// Sets the least amount of minimum width the window should be resized.
    pub fn min_width(mut self, min_width: u32) -> Self {
        self.min_width = min_width;
        self
    }

    /// Sets the window to the maximum available size.
    ///
    /// # Errors
    /// Calling this method has no effect when the application cannot determine the screen size.
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
    pub fn ui<F: 'static + Fn(&mut World) -> Entity>(mut self, ui_builder: F) -> Self {
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

        let mut max_width = 65535;
        let mut max_height = 65535;

        if !self.max_height_set || !self.max_width_set {
            if let Ok(max_size) = orbclient::get_display_size() {
                if self.max_height_set {
                    max_height = max_size.1;
                }
                if self.max_width_set {
                    max_width = max_size.0;
                }
            }
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
            max_height: max_height,
            max_width: max_width,
            min_height: self.min_height,
            min_width: self.min_width,
            ui: self.ui,
        }
    }
}
