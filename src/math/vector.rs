pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(pos_x: f32, pos_y: f32) -> Vector {
        Vector { x: pos_x, y: pos_y }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn set_pos(&mut self, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
    }
}
