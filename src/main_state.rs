extern crate ggez;
use ggez::event;
use ggez::event::Keycode;
use ggez::event::Mod;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::rc::Rc;
use std::time::Duration;
use states::menu_state::MenuState;
use states::play_state::PlayState;
use std::collections::HashMap;
use states::GameState;
// First we make a structure to contain the game's state
pub struct MainState {
    frames: usize,
    states: Vec<Box<GameState>>,
    state: usize,
    keys: HashMap<Keycode, bool>,
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = Rc::new(graphics::Font::new(ctx, "/OpenSans-Regular.ttf", 48)?);
        let s = MainState {
            frames: 0,
            keys: HashMap::new(),
            states: vec![Box::new(MenuState::new(&font, ctx)), Box::new(PlayState::new(ctx))],
            state: 0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.state = self.states[self.state].update(&mut self.keys, _ctx, _dt)?;
        self.keys.clear();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        (&self).states[self.state].draw(&self, ctx)?;
        // Drawables are drawn from their center.
        graphics::present(ctx);
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _: Mod, _: bool) {
        self.keys.insert(keycode, true);
    }

    fn key_up_event(&mut self, keycode: Keycode, _: Mod, _: bool) {
        self.keys.insert(keycode, false);
    }
}
