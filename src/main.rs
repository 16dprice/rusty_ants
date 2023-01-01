use ggez::{event, GameResult};
use global_state::GlobalState;

pub mod global_state;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    let state = GlobalState::new()?;
    event::run(ctx, event_loop, state)
}
