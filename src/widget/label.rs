use specs::{Builder, Entity, World, WorldExt};
use crate::{component::TextComponent};

pub struct Label {
    entity: Entity,
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
        world.register::<TextComponent>();
        let text_component = TextComponent {
            text: self.text
        };
        let label = world.create_entity().with(text_component.clone()).build();
        Label {
            entity: label,
            text: text_component
        }
    }
}