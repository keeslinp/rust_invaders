extern crate ggez;
extern crate uuid;
use ggez::event::Keycode;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::time::Duration;
use main_state::MainState;
use std::collections::HashMap;
use states::GameState;
use states::entities::entity::Entity;
use states::entities::player::new_player;
use states::components::position::Position;
use uuid::Uuid;

struct World {
    components: HashMap<Uuid, Position>,
    entities: Vec<Entity>,
}

pub struct PlayState {
    text: graphics::Text,
    world: World,
}

impl PlayState {
    pub fn new(font: &graphics::Font, ctx: &mut Context) -> Self {
        let mut components = HashMap::new();
        let entities = vec![new_player(&mut components, 0.0, 0.0)];
        PlayState {
            text: graphics::Text::new(ctx, "Testing", &font).unwrap(),
            world: World {
                components,
                entities,
            },
        }
    }
}

impl GameState for PlayState{
    fn update(&mut self, keys: &mut HashMap<Keycode, bool>, ctx: &mut Context, _: Duration) -> GameResult<(usize)> {
        for (key, value) in keys.drain() {
            if value {
                return Ok(0);
            }
        }
        Ok(1)
    }
    fn draw(&self, state: &MainState, ctx: &mut Context) -> GameResult<()> {
        let dest_point = graphics::Point::new(200.0, 50.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;
        Ok(())
    }
}
