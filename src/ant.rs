use crate::math::vector::Vector;
use rand::Rng;

pub struct Ant {
    pos: Vector,
}

impl Ant {
    pub fn new(pos_x: f32, pos_y: f32) -> Ant {
        Ant {
            pos: Vector::new(pos_x, pos_y),
        }
    }

    pub fn get_x(&self) -> f32 {
        self.pos.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.pos.get_y()
    }

    pub fn update(&mut self) {
        let mut range = rand::thread_rng();

        let rand_x = range.gen_range(-1.0, 1.0);
        let rand_y = range.gen_range(-1.0, 1.0);

        let new_x = self.get_x() + rand_x;
        let new_y = self.get_y() + rand_y;

        self.pos.set_pos(new_x, new_y);
    }
}
