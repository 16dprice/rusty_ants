use ant::Ant;
use food::Food;
use ggez::{event, GameResult};
use global_state::GlobalState;
use pheromone::Pheromone;
use rand::Rng;

const NUM_ANTS: u16 = 4;
const NUM_FOOD: u16 = 10;
const NUM_PHEROMONES: u16 = 20;

pub mod ant;
pub mod food;
pub mod global_state;
pub mod math;
pub mod pheromone;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    let mut range = rand::thread_rng();

    let mut ants = vec![];
    for _ in 0..NUM_ANTS {
        ants.push(Ant::new(
            range.gen_range(100.0, 700.0),
            range.gen_range(100.0, 500.0),
        ));
    }

    let mut pheromones = vec![];
    for _ in 0..NUM_PHEROMONES {
        pheromones.push(Pheromone::new(
            range.gen_range(100.0, 700.0),
            range.gen_range(100.0, 500.0),
        ));
    }

    let mut food = vec![];
    for _ in 0..NUM_FOOD {
        food.push(Food::new(
            range.gen_range(100.0, 700.0),
            range.gen_range(100.0, 500.0),
        ));
    }

    let state = GlobalState::new(ants, pheromones, food)?;

    event::run(ctx, event_loop, state)
}
