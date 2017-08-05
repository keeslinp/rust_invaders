extern crate ggez;
use states::menu_items::MENU_ITEMS;
use ggez::event::Keycode;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::time::Duration;
use main_state::MainState;
use std::collections::HashMap;
use states::GameState;

pub struct MenuState {
    menu_choice: usize,
    menu_items: Vec<graphics::Text>,
}

impl MenuState {
    pub fn new(font: &graphics::Font, ctx: &mut Context) -> Self {
        let menu_items = MENU_ITEMS.iter().map(|val| graphics::Text::new(ctx, &val, &font).unwrap()).collect();
        MenuState {
            menu_choice: 0,
            menu_items,
        }
    }
}

impl GameState for MenuState {

    fn update(&mut self, keys: &mut HashMap<Keycode, bool>, ctx: &mut Context, _: Duration) -> GameResult<(usize)> {
        for (key, value) in keys.drain() {
            match (key, value) {
                (Keycode::Down, true) => {
                    self.menu_choice = if self.menu_choice >= self.menu_items.len() - 1 {
                        0
                    } else {
                        self.menu_choice + 1
                    }
                },
                (Keycode::Up, true) => {
                    self.menu_choice = if self.menu_choice <= 0 {
                        self.menu_items.len() - 1
                    } else {
                        self.menu_choice - 1
                    }
                },
                (Keycode::Return, true) => {
                    if self.menu_choice == 2 {
                        ctx.quit()?;
                    }
                    if self.menu_choice == 0 {
                        return Ok(1);
                    }
                }
                _ => (),
            }
        }

        Ok(0)
    }

    fn draw(&self, state: &MainState, ctx: &mut Context) -> GameResult<()> {
        for (index, item) in self.menu_items.iter().enumerate() {
            let width = item.width() as f32;
            let height = item.height() as f32;
            let x_pos = width / 2.0 + 10.0;
            let y_pos = height / 2.0 + (100.0 * index as f32);
            let dest_point = graphics::Point::new(x_pos, y_pos);
            graphics::draw(ctx, item, dest_point, 0.0)?;
            if self.menu_choice == index {
                let rect = graphics::Rect::new(x_pos, y_pos, width, height);
                graphics::rectangle(ctx, graphics::DrawMode::Line, rect)?;
            }
        }
        Ok(())
    }
}
