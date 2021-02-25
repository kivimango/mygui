use specs::{Builder, Entity, World, WorldExt};
use crate::{PositionComponent, RenderComponent, Shape, SizeComponent, component::TextComponent};

const LABEL_DEFAULT_WIDTH: usize = 150;
const LABEL_DEFAULT_HEIGHT: usize = 55;

pub struct Label {
    entity: Entity,
    position: PositionComponent,
    render: RenderComponent,
    size: SizeComponent,
    text: TextComponent,
}

impl Label {
    pub fn new(text: String) -> LabelBuilder {
        LabelBuilder::new()
        .text(text)
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn position(&self) -> PositionComponent {
        self.position
    }

    pub fn size(&self) -> SizeComponent {
        self.size
    }

    pub fn text(&self) -> &TextComponent {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut TextComponent {
        &mut self.text
    }

    pub fn set_text(&mut self, new_text: String) {
        self.text = TextComponent {
            text: new_text
        };
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
                shape: Shape::Rectangle
            },
            size: SizeComponent {
                width: LABEL_DEFAULT_WIDTH,
                height: LABEL_DEFAULT_HEIGHT
            },
            text: TextComponent::default(),
        }
    }

    pub fn position(mut self, x: usize, y: usize) -> LabelBuilder {
        self.position = PositionComponent {
            x,
            y
        };
        self
    }

    pub fn size(mut self, width: usize, height: usize) -> LabelBuilder {
        self.size = SizeComponent {
            width,
            height
        };
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = TextComponent {
            text
        };
        self
    }

    pub fn build(self, world: &mut World) -> Label {
        world.register::<PositionComponent>();
        world.register::<RenderComponent>();
        world.register::<TextComponent>();
        world.register::<SizeComponent>();

        let label = world.create_entity()
            .with(self.position)
            .with(self.render)
            .with(self.size)
            .with(self.text.clone())
            .build();

        Label {
            entity: label,
            render: self.render,
            position: self.position,
            size: self.size,
            text: self.text
        }
    }
}