use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

pub struct GlobalState {
    pos_x: f32,
    pos_y: f32,
}

impl GlobalState {
    pub fn new() -> GameResult<GlobalState> {
        let s = GlobalState {
            pos_x: 0.0,
            pos_y: 0.0,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GlobalState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        self.pos_y = self.pos_x;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.9, 0.1, 0.3, 1.0]));

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::BLACK,
        )?;
        canvas.draw(&circle, Vec2::new(self.pos_x, self.pos_y));

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
        println!("({}, {})", _x, _y);

        Ok(())
    }
}
