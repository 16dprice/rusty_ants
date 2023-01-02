use crate::math::vector::Vector;

pub struct Food {
    pos: Vector,
}

impl Food {
    pub fn new(pos_x: f32, pos_y: f32) -> Food {
        Food {
            pos: Vector::new(pos_x, pos_y),
        }
    }

    pub fn get_x(&self) -> f32 {
        self.pos.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.pos.get_y()
    }
}
