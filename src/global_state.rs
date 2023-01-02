use crate::ant::Ant;
use crate::food::Food;
use crate::math::vector::Vector;
use crate::pheromone::Pheromone;

pub struct GlobalState {
    ants: Vec<Ant>,
    food: Vec<Food>,
    pheromones: Vec<Pheromone>,
    target: Vector,
}

impl GlobalState {
    pub fn new(ants: Vec<Ant>, pheromones: Vec<Pheromone>, food: Vec<Food>) -> GlobalState {
        GlobalState {
            ants,
            pheromones,
            food,
            target: Vector::new(300.0, 300.0),
        }
    }
}
