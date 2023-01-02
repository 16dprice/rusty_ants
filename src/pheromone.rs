use crate::math::vector::Vector;
use std::time::Instant;

const EXPIRATION_TIME: f32 = 10_000.0;

#[derive(Copy, Clone)]
enum PheromoneType {
    Food,
    GoHome,
}

#[derive(Copy, Clone)]
pub struct Pheromone {
    pos: Vector,
    pheromone_type: PheromoneType,
    created_at: Instant,
}

impl Pheromone {
    pub fn new(pos_x: f32, pos_y: f32) -> Pheromone {
        Pheromone {
            pos: Vector::new(pos_x, pos_y),
            pheromone_type: PheromoneType::GoHome,
            created_at: Instant::now(),
        }
    }

    pub fn get_x(&self) -> f32 {
        self.pos.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.pos.get_y()
    }

    pub fn percent_time_left(&self) -> f32 {
        let time_alive = self.created_at.elapsed().as_millis();
        if time_alive > (EXPIRATION_TIME as u128) {
            return 0.0;
        } else {
            return 1.0 - ((time_alive as f32) / EXPIRATION_TIME);
        }
    }
}
