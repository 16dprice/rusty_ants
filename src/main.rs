use ant::Ant;
use food::Food;
use global_state::GlobalState;
use pheromone::Pheromone;
use rand::Rng;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const NUM_ANTS: u16 = 10;
const NUM_FOOD: u16 = 10;
const NUM_PHEROMONES: u16 = 0;

pub mod ant;
pub mod food;
pub mod global_state;
pub mod math;
pub mod pheromone;

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}

pub fn main() {
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

    let state = GlobalState::new(ants, pheromones, food);

    run();
}
