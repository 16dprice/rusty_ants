use crate::{math::vector::Vector, pheromone::Pheromone};
use rand::Rng;

const TOL: f32 = 2.0;

const PI_OVER_TWO: f32 = 1.570796327;
const PI: f32 = 3.141592654;
const THREE_PI_OVER_TWO: f32 = 4.712388980;
const TWO_PI: f32 = 6.283185307;
const FIVE_PI_OVER_TWO: f32 = 7.853981634;

enum Position {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

pub struct Ant {
    pos: Vector,
    desired_direction: Vector,
    velocity: Vector,
    speed: f32,
    steer_strength: f32,
    wander_strength: f32,
}

impl Ant {
    pub fn new(pos_x: f32, pos_y: f32) -> Ant {
        Ant {
            pos: Vector::new(pos_x, pos_y),
            desired_direction: Vector::right(),
            velocity: Vector::right(),
            speed: 1.0,
            steer_strength: 2.0,
            wander_strength: 0.2,
        }
    }

    pub fn get_x(&self) -> f32 {
        self.pos.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.pos.get_y()
    }

    pub fn update(&mut self) {
        let random_direction = self.direction().times(self.wander_strength);
        self.desired_direction = self.desired_direction.add(random_direction).normalize();

        let desired_velocity = self.desired_direction.times(self.speed);
        let desired_steering_force = desired_velocity
            .sub(self.velocity)
            .times(self.steer_strength);
        let acceleration = desired_steering_force.clamp_magnitude(self.steer_strength);

        self.velocity = self.velocity.add(acceleration).clamp_magnitude(self.speed);
        self.pos = self.pos.add(self.velocity);
    }

    fn direction(&self) -> Vector {
        match self.position() {
            None => Vector::random_in_unit_circle(),
            Some(Position::TopLeft) => Vector::random_in_angle_range(THREE_PI_OVER_TWO, TWO_PI),
            Some(Position::Top) => Vector::random_in_angle_range(PI, TWO_PI),
            Some(Position::TopRight) => Vector::random_in_angle_range(PI, THREE_PI_OVER_TWO),
            Some(Position::Right) => Vector::random_in_angle_range(PI_OVER_TWO, THREE_PI_OVER_TWO),
            Some(Position::BottomRight) => Vector::random_in_angle_range(PI_OVER_TWO, PI),
            Some(Position::Bottom) => Vector::random_in_angle_range(0.0, PI),
            Some(Position::BottomLeft) => Vector::random_in_angle_range(0.0, PI_OVER_TWO),
            Some(Position::Left) => {
                Vector::random_in_angle_range(THREE_PI_OVER_TWO, FIVE_PI_OVER_TWO)
            }
        }
    }

    fn position(&self) -> Option<Position> {
        if self.get_x() < 50.0 {
            if self.get_y() < 50.0 {
                return Some(Position::TopLeft);
            } else if self.get_y() > 550.0 {
                return Some(Position::BottomLeft);
            } else {
                return Some(Position::Left);
            }
        } else if self.get_x() > 750.0 {
            if self.get_y() < 50.0 {
                return Some(Position::TopRight);
            } else if self.get_y() > 550.0 {
                return Some(Position::BottomRight);
            } else {
                return Some(Position::Right);
            }
        } else if self.get_y() < 50.0 {
            return Some(Position::Top);
        } else if self.get_y() > 550.0 {
            return Some(Position::Bottom);
        }

        None
    }
}
