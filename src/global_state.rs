use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

use crate::ant::Ant;
use crate::math::vector::Vector;
use crate::pheromone::Pheromone;

pub struct GlobalState {
    ants: Vec<Ant>,
    pheromones: Vec<Pheromone>,
    target: Vector,
}

impl GlobalState {
    pub fn new(ants: Vec<Ant>, pheromones: Vec<Pheromone>) -> GameResult<GlobalState> {
        let s = GlobalState {
            ants,
            pheromones,
            target: Vector::new(300.0, 300.0),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GlobalState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for ant in self.ants.iter_mut() {
            // ant.update(&self.pheromones);
            ant.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.9, 0.1, 0.3, 1.0]));

        // for pheromone in self.pheromones.iter() {
        //     let circle = graphics::Mesh::new_circle(
        //         ctx,
        //         graphics::DrawMode::fill(),
        //         Vec2::new(0.0, 0.0),
        //         pheromone.get_range(),
        //         2.0,
        //         Color::MAGENTA,
        //     )?;

        //     canvas.draw(&circle, Vec2::new(pheromone.get_x(), pheromone.get_y()))
        // }

        for ant in self.ants.iter() {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                6.0,
                5.0,
                Color::BLACK,
            )?;

            canvas.draw(&circle, Vec2::new(ant.get_x(), ant.get_y()))
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        self.target = Vector::new(_x, _y);
        println!("({}, {})", _x, _y);

        Ok(())
    }
}
