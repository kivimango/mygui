use std::usize;

use crate::{Border, PositionComponent, RenderComponent, Shape, SizeComponent, component::TextComponent};
use orbclient::Color;
use specs::{Builder, Entity, World, WorldExt};

const LABEL_DEFAULT_WIDTH: usize = 150;
const LABEL_DEFAULT_HEIGHT: usize = 55;

pub struct Label {}

impl Label {
    pub fn new(text: String) -> LabelBuilder {
        LabelBuilder::new().text(text)
    }
}

pub struct LabelBuilder {
    background: Option<Color>,
    border: Option<Border>,
    font_family: String,
    font_size: usize,
    position: PositionComponent,
    size: SizeComponent,
    text: String,
    text_color: orbclient::Color
}

impl LabelBuilder {
    pub fn new() -> LabelBuilder {
        LabelBuilder {
            background: None,
            border: None,
            font_family: "Roboto-Medium".to_string(),
            font_size: 12,
            position: PositionComponent::default(),
            size: SizeComponent {
                width: LABEL_DEFAULT_WIDTH,
                height: LABEL_DEFAULT_HEIGHT,
            },
            text: String::new(),
            text_color: orbclient::Color::rgba(255, 255, 255, 255)
        }
    }

    pub fn background(mut self, bg_color: Color) -> LabelBuilder {
        self.background = Some(bg_color);
        self
    }

    pub fn border(mut self, border: Border) -> LabelBuilder {
        self.border = Some(border);
        self
    }

    pub fn font_family(mut self, font: String) -> LabelBuilder {
        self.font_family = font;
        self
    }

    pub fn font_size(mut self, font_size: usize) -> LabelBuilder {
        self.font_size = font_size;
        self
    }

    pub fn position(mut self, x: u32, y: u32) -> LabelBuilder {
        self.position = PositionComponent { x, y };
        self
    }

    pub fn size(mut self, width: usize, height: usize) -> LabelBuilder {
        self.size = SizeComponent { width, height };
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn text_color(mut self, color: orbclient::Color) -> LabelBuilder {
        self.text_color = color;
        self
    }

    pub fn build(self, world: &mut World) -> Entity {
        world.register::<PositionComponent>();
        world.register::<RenderComponent>();
        world.register::<TextComponent>();
        world.register::<SizeComponent>();

        let render_component = RenderComponent {
            background: self.background,
            border: self.border,
            shape: Shape::Rectangle,
        };
        let text = TextComponent {
            font_family: self.font_family.to_string(),
            font_size: self.font_size,
            text: self.text,
            text_color: self.text_color
        };

        world
            .create_entity()
            .with(self.position)
            .with(render_component)
            .with(self.size)
            .with(text)
            .build()
    }
}
