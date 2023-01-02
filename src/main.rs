use ant::Ant;
use ggez::{event, GameResult};
use global_state::GlobalState;
use pheromone::Pheromone;
use rand::Rng;

pub mod ant;
pub mod global_state;
pub mod math;
pub mod pheromone;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    let mut range = rand::thread_rng();

    let mut ants = vec![];
    for _ in 1..100 {
        ants.push(Ant::new(
            range.gen_range(100.0, 700.0),
            range.gen_range(100.0, 500.0),
        ));
    }

    // let pheromones = vec![
    //     Pheromone::new(86.0, 509.0),
    //     Pheromone::new(78.0, 479.0),
    //     Pheromone::new(83.0, 450.0),
    //     Pheromone::new(118.0, 439.0),
    //     Pheromone::new(150.0, 401.0),
    //     Pheromone::new(160.0, 394.0),
    //     Pheromone::new(170.0, 360.0),
    //     Pheromone::new(180.0, 344.0),
    //     Pheromone::new(190.0, 321.0),
    //     Pheromone::new(210.0, 321.0),
    //     Pheromone::new(225.0, 321.0),
    //     Pheromone::new(250.0, 321.0),
    //     Pheromone::new(270.0, 345.0),
    //     Pheromone::new(280.0, 330.0),
    // ];

    let state = GlobalState::new(ants, vec![])?;

    event::run(ctx, event_loop, state)
}
