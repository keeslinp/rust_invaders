use ggez::{GameResult, Context};
use ggez::graphics;
pub trait Drawable {
    fn get_center(&self) -> (f32, f32);

    fn get_rect(&self) -> graphics::Rect;

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::rectangle(ctx, graphics::DrawMode::Line, self.get_rect())?;
        Ok(())
    }
}
