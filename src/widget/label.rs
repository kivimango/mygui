use crate::{component::TextComponent, PositionComponent, RenderComponent, Shape, SizeComponent};
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
    position: PositionComponent,
    render: RenderComponent,
    size: SizeComponent,
    text: TextComponent,
}

impl LabelBuilder {
    pub fn new() -> LabelBuilder {
        LabelBuilder {
            position: PositionComponent::default(),
            render: RenderComponent {
                shape: Shape::Rectangle,
            },
            size: SizeComponent {
                width: LABEL_DEFAULT_WIDTH,
                height: LABEL_DEFAULT_HEIGHT,
            },
            text: TextComponent::default(),
        }
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
        self.text = TextComponent { text };
        self
    }

    pub fn build(self, world: &mut World) -> Entity {
        world.register::<PositionComponent>();
        world.register::<RenderComponent>();
        world.register::<TextComponent>();
        world.register::<SizeComponent>();

        world
            .create_entity()
            .with(self.position)
            .with(self.render)
            .with(self.size)
            .with(self.text.clone())
            .build()
    }
}
