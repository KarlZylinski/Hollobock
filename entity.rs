use rsfml::graphics::{RenderWindow};

pub trait Entity {
	fn update(&self, dt: f32) -> ~Entity;
	fn draw(&self, window: &mut RenderWindow);
}
