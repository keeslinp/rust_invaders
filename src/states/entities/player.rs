extern crate uuid;
use uuid::Uuid;
use states::entities::entity::Entity;
use states::components::position::Position;
use std::collections::HashMap;

pub fn new_player(components: &mut HashMap<Uuid, Position>, start_x: f32, start_y: f32) -> Entity {
    let position_component = Position::new(start_x, start_y);
    let id: Uuid = Uuid::new_v4();
    let mut component_ids = Vec::with_capacity(1);
    components.insert(id, position_component);
    component_ids.push(id);
    Entity::new(component_ids)
}
