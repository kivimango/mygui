use orbclient::Color;

/// Defines a rectangle border around widgets.
#[derive(Copy, Clone, Debug)]
pub struct Border {
    pub color: orbclient::Color,
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub radius: f32,
}

impl Border {
    pub fn new() -> BorderBuilder {
        BorderBuilder::new()
    }
}

impl Default for Border {
    /// Creates a default rectangle Border instance with a black color and a 1 pixel of width.
    fn default() -> Self {
        Border {
            color: Color::rgba(0, 0, 0, 255),
            left: 1.0,
            top: 1.0,
            right: 1.0,
            bottom: 1.0,
            radius: 0.0,
        }
    }
}

pub struct BorderBuilder {
    color: orbclient::Color,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    radius: f32,
}

impl BorderBuilder {
    pub fn new() -> BorderBuilder {
        BorderBuilder {
            color: Color::rgba(0, 0, 0, 255),
            left: 1.0,
            top: 1.0,
            right: 1.0,
            bottom: 1.0,
            radius: 0.0,
        }
    }

    pub fn color(mut self, color: orbclient::Color) -> BorderBuilder {
        self.color = color;
        self
    }

    pub fn left(mut self, width: f32) -> BorderBuilder {
        self.left = width;
        self
    }

    pub fn top(mut self, width: f32) -> BorderBuilder {
        self.top = width;
        self
    }

    pub fn right(mut self, width: f32) -> BorderBuilder {
        self.right = width;
        self
    }

    pub fn bottom(mut self, width: f32) -> BorderBuilder {
        self.bottom = width;
        self
    }

    pub fn width(mut self, width: f32) -> BorderBuilder {
        self.left = width;
        self.top = width;
        self.right = width;
        self.bottom = width;
        self
    }

    pub fn radius(mut self, radius: f32) -> BorderBuilder {
        self.radius = radius;
        self
    }

    pub fn build(self) -> Border {
        Border {
            color: self.color,
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            radius: self.radius,
        }
    }
}

