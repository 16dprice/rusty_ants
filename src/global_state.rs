use ggez::event;
use ggez::glam::*;
use ggez::graphics::Canvas;
use ggez::graphics::{self, Color};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::ant::Ant;
use crate::food::Food;
use crate::math::vector::Vector;
use crate::pheromone::Pheromone;

pub struct GlobalState {
    ants: Vec<Ant>,
    food: Vec<Food>,
    pheromones: Vec<Pheromone>,
    target: Vector,
}

impl GlobalState {
    pub fn new(
        ants: Vec<Ant>,
        pheromones: Vec<Pheromone>,
        food: Vec<Food>,
    ) -> GameResult<GlobalState> {
        let s = GlobalState {
            ants,
            pheromones,
            food,
            target: Vector::new(300.0, 300.0),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GlobalState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for ant in self.ants.iter_mut() {
            let pheromone_opt = ant.update();

            match pheromone_opt {
                None => {}
                Some(pheromone) => {
                    println!("{}", self.pheromones.len());
                    self.pheromones.push(pheromone)
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.5, 0.5, 0.5, 1.0]));

        canvas = draw_food(&self.food, ctx, canvas)?;
        canvas = draw_pheromones(&self.pheromones, ctx, canvas)?;
        canvas = draw_ants(&self.ants, ctx, canvas)?;

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

fn draw_food(foods: &Vec<Food>, ctx: &mut Context, mut canvas: Canvas) -> GameResult<Canvas> {
    for food in foods.iter() {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            6.0,
            5.0,
            Color::GREEN,
        )?;
        canvas.draw(&circle, Vec2::new(food.get_x(), food.get_y()));
    }

    Ok(canvas)
}

fn draw_pheromones(
    pheromones: &Vec<Pheromone>,
    ctx: &mut Context,
    mut canvas: Canvas,
) -> GameResult<Canvas> {
    for pheromone in pheromones.iter() {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            6.0,
            2.0,
            Color::from([1.0, 0.0, 1.0, pheromone.percent_time_left()]),
        )?;

        canvas.draw(&circle, Vec2::new(pheromone.get_x(), pheromone.get_y()));
    }

    Ok(canvas)
}

fn draw_ants(ants: &Vec<Ant>, ctx: &mut Context, mut canvas: Canvas) -> GameResult<Canvas> {
    for ant in ants.iter() {
        let ant_x = ant.get_x();
        let ant_y = ant.get_y();

        let vel_x = ant.get_velocity().get_x();
        let vel_y = ant.get_velocity().get_y();

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            6.0,
            5.0,
            Color::BLACK,
        )?;
        canvas.draw(&circle, Vec2::new(ant_x, ant_y));

        let pt1 = Point2::from_slice(&[ant_x, ant_y]);
        let pt2 = Point2::from_slice(&[ant_x + 10.0 * vel_x, ant_y + 10.0 * vel_y]);

        let line = graphics::Mesh::new_line(ctx, &[pt1, pt2], 2.0, Color::BLACK)?;

        canvas.draw(&line, Vec2::new(0.0, 0.0));
    }

    Ok(canvas)
}
