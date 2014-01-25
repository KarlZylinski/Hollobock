use rsfml::graphics::{RenderWindow};
use list;
use input::{Input};

pub trait Entity {
	fn update(&self, dt: f32, input: &Input) -> UpdateResult;
	fn draw(&self, window: &mut RenderWindow);
}

pub struct UpdateResult {
	new_entities: list::List<~Entity>
	// TODO: Add events here.
}
