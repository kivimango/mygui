use specs::{Builder, Entity, World, WorldExt};
use crate::{PositionComponent, RenderComponent, Shape, SizeComponent, component::TextComponent};

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
    text: String,
}

impl LabelBuilder {
    pub fn new() -> LabelBuilder {
        LabelBuilder {
            text: String::default()
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn build(self, world: &mut World) -> Label {
        world.register::<PositionComponent>();
        world.register::<RenderComponent>();
        world.register::<TextComponent>();
        world.register::<SizeComponent>();

        let position = PositionComponent {
            x:5,
            y:5
        };
        let shape = RenderComponent {
            shape: Shape::Rectangle
        };
        let size = SizeComponent {
            width: 100,
            height: 55
        };
        let text = TextComponent {
            text: self.text
        };
        let label = world.create_entity()
            .with(position.clone())
            .with(shape.clone())
            .with(text.clone())
            .with(size.clone())
            .build();
        Label {
            entity: label,
            render: shape,
            position,
            size,
            text
        }
    }
}