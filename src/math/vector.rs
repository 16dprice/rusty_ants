use rand::Rng;

const TWO_PI: f32 = 6.283185307;

#[derive(Copy, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(pos_x: f32, pos_y: f32) -> Vector {
        Vector { x: pos_x, y: pos_y }
    }

    pub fn new_normalized(pos_x: f32, pos_y: f32) -> Vector {
        Vector::new(pos_x, pos_y).normalize()
    }

    pub fn right() -> Vector {
        Vector::new(1.0, 0.0)
    }

    pub fn random_in_unit_circle() -> Vector {
        let mut range = rand::thread_rng();
        let dir = range.gen_range(0.0, TWO_PI);

        Vector::new(dir.cos(), dir.sin())
    }

    pub fn random_in_angle_range(theta_min: f32, theta_max: f32) -> Vector {
        let mut range = rand::thread_rng();
        let dir = range.gen_range(theta_min, theta_max);

        Vector::new(dir.cos(), -dir.sin())
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

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn normalize(&self) -> Vector {
        let length = self.length();
        Vector::new(self.x / length, self.y / length)
    }

    pub fn add(&self, addend: Vector) -> Vector {
        Vector::new(self.x + addend.get_x(), self.y + addend.get_y())
    }

    pub fn sub(&self, subtrahend: Vector) -> Vector {
        Vector::new(self.x - subtrahend.get_x(), self.y - subtrahend.get_y())
    }

    pub fn times(&self, scalar: f32) -> Vector {
        Vector::new(scalar * self.x, scalar * self.y)
    }

    pub fn clamp_magnitude(&self, clamp: f32) -> Vector {
        let length = self.length();

        if length <= clamp {
            return Vector::new(self.x, self.y);
        }

        let scale = clamp / length;
        Vector::new(scale * self.x, scale * self.y)
    }
}
