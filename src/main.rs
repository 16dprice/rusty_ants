use ant::Ant;
use ggez::{event, GameResult};
use global_state::GlobalState;

pub mod ant;
pub mod global_state;
pub mod math;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    let ants = vec![
        Ant::new(300.0, 300.0),
        Ant::new(400.0, 300.0),
        Ant::new(500.0, 300.0),
        Ant::new(600.0, 300.0),
    ];

    let state = GlobalState::new(ants)?;

    event::run(ctx, event_loop, state)
}
