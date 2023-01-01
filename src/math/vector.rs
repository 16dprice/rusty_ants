pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(pos_x: f32, pos_y: f32) -> Vector {
        Vector { x: pos_x, y: pos_y }
    }
}
