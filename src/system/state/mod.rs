mod context;
mod state;

pub use self::context::*;
pub use self::state::*;

pub trait State {
    fn init(&mut self, ctx: &mut Context);
    fn message(&mut self, ctx:&mut Context);
    fn update(&mut self, ctx: &mut Context);
}