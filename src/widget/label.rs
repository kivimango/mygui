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
    position: PositionComponent,
    size: SizeComponent,
    text: String,
}

impl LabelBuilder {
    pub fn new() -> LabelBuilder {
        LabelBuilder {
            background: None,
            border: None,
            position: PositionComponent::default(),
            size: SizeComponent {
                width: LABEL_DEFAULT_WIDTH,
                height: LABEL_DEFAULT_HEIGHT,
            },
            text: String::new(),
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

    pub fn position(mut self, x: usize, y: usize) -> LabelBuilder {
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
            text: self.text
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
