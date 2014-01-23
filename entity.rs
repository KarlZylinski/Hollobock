use rsfml::graphics::{RenderWindow};
use list;

pub trait Entity {
	fn update(&self, dt: f32) -> UpdateResult;
	fn draw(&self, window: &mut RenderWindow);
}

pub struct UpdateResult {
	new_entities: list::List<~Entity>
	// TODO: Add events here.
}
