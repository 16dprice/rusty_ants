use crate::math::vector::Vector;

pub struct Ant {
    pub pos: Vector,
}

impl Ant {
    pub fn new(pos_x: f32, pos_y: f32) -> Ant {
        Ant {
            pos: Vector::new(pos_x, pos_y),
        }
    }
}
