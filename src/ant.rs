use crate::math::vector::Vector;
use rand::Rng;

const TOL: f32 = 0.0001;

pub struct Ant {
    pos: Vector,
    speed: f32,
}

impl Ant {
    pub fn new(pos_x: f32, pos_y: f32) -> Ant {
        let mut range = rand::thread_rng();

        Ant {
            pos: Vector::new(pos_x, pos_y),
            speed: range.gen_range(0.0, 1.0),
        }
    }

    pub fn get_x(&self) -> f32 {
        self.pos.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.pos.get_y()
    }

    pub fn update(&mut self, target: Vector) {
        let delta_x = target.get_x() - self.get_x();
        let delta_y = target.get_y() - self.get_y();

        if f32::abs(delta_x) < TOL && f32::abs(delta_y) < TOL {
            return;
        }

        let direction = Vector::new_normalized(delta_x, delta_y);

        let new_x = self.get_x() + self.speed * direction.get_x();
        let new_y = self.get_y() + self.speed * direction.get_y();

        self.pos.set_pos(new_x, new_y);
    }
}
