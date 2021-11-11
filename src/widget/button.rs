use std::usize;
use orbclient::{ButtonEvent, Color};
use specs::{Builder, Entity, World, WorldExt};
use crate::{Context, MouseClickHandler, OnClickHandler, PositionComponent, RenderComponent, Shape, SizeComponent, State, StateComponent, TextComponent};

pub struct Button;

impl Button {
    pub fn new() -> ButtonBuilder {
        ButtonBuilder::new()
    }
}

pub struct ButtonBuilder {
    enabled: bool,
    font_family: String,
    font_size: usize,
    text_color: orbclient::Color,
    title: String,
    on_click_handler: Option<Box<OnClickHandler>>
}

impl ButtonBuilder {
    pub fn new() -> Self {
        ButtonBuilder {
            enabled: true,
            font_family: "Roboto-Medium".to_string(),
            font_size: 12,
            text_color: orbclient::Color::rgba(0, 0, 0, 255),
            title: String::new(),
            on_click_handler: None
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    pub fn on_click<F: 'static + FnMut(&mut Context, ButtonEvent)>(mut self, f: F) -> Self {
        self.on_click_handler = Some(Box::new(f));
        self
    }

    pub fn build(self, world: &mut World) -> Entity {
        world.register::<MouseClickHandler>();
        world.register::<StateComponent>();
        world.register::<TextComponent>();

        world.register::<PositionComponent>();
        world.register::<RenderComponent>();
        world.register::<SizeComponent>();

        let text = TextComponent {
            text: self.title,
            font_family: self.font_family,
            font_size: self.font_size,
            text_color: self.text_color,
        };
        
        let state = StateComponent {
            enabled: self.enabled,
            state_object: Box::new(ButtonState::new())
        };

        let render_component = RenderComponent {
            background: Some(Color::rgb(255, 255, 255)),
            border: None,
            shape: Shape::Rectangle,
        };

        let pos = PositionComponent {
            x: 25,
            y: 60
        };

        let size = SizeComponent {
            width: 125,
            height: 36
        };

        let on_click_handler = MouseClickHandler {
            on_click_handler: self.on_click_handler.expect("You must define an on_click handler on a Button!")
        };

        world.create_entity()
        .with(state)
        .with(text)
        .with(render_component)
        .with(pos)
        .with(size)
        .with(on_click_handler)
        .build()
    }
}

struct ButtonState {
    _enabled: bool,
}

impl ButtonState {
    fn new() -> Self {
        ButtonState {
            _enabled: true,
        }
    }
}

impl State for ButtonState {
    fn init(&mut self, _ctx: &mut Context) {
        println!("Init");
    }

    fn message(&mut self, _ctx:&mut Context) {
        todo!()
    }

    fn update(&mut self, _ctx: &mut Context) {
        println!("updating...");
    }
}
