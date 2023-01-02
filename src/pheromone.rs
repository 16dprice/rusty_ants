use crate::math::vector::Vector;

const DEFAULT_PHEROMONE_RANGE: f32 = 20.0;

#[derive(Copy, Clone)]
pub struct Pheromone {
    pos: Vector,
    range: f32,
}

impl Pheromone {
    pub fn new(pos_x: f32, pos_y: f32) -> Pheromone {
        Pheromone {
            pos: Vector::new(pos_x, pos_y),
            range: DEFAULT_PHEROMONE_RANGE,
        }
    }

    pub fn get_x(&self) -> f32 {
        self.pos.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.pos.get_y()
    }

    pub fn get_range(&self) -> f32 {
        self.range
    }

    fn get_strength(self, distance: f32) -> f32 {
        (distance / (1.0 - self.range)) + (self.range / (self.range - 1.0))
    }

    pub fn get_strength_vector(self, initial: Vector) -> Vector {
        let delta_x = self.get_x() - initial.get_x();
        let delta_y = self.get_y() - initial.get_y();

        let distance = f32::sqrt(delta_x * delta_x + delta_y * delta_y);

        if distance <= 1.0 {
            return Vector::new(delta_x, delta_y);
        } else if distance > self.range {
            return Vector::new(0.0, 0.0);
        } else {
            let strength = self.get_strength(distance);
            return Vector::new(strength * delta_x, strength * delta_y);
        }
    }
}
