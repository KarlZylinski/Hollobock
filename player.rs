use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, RectangleShape};
use rsfml::window::{keyboard};

use entity::{Entity};

pub struct Player {
	position: Vector2f
}

fn input_vector() -> Vector2f {
	return Vector2f::new(
		if keyboard::is_key_pressed(keyboard::Left) {
			-1.
		} else if keyboard::is_key_pressed(keyboard::Right) {
			1.
		} else {
			0.
		},
		if keyboard::is_key_pressed(keyboard::Up) {
			-1.
		} else if keyboard::is_key_pressed(keyboard::Down) {
			1.
		} else {
			0.
		}
	);
}

impl Entity for Player {
	

	fn update(&self, dt: f32) -> ~Entity {
		return ~Player { position: self.position + input_vector() * 200.0f32 * dt } as ~Entity;
	}

	fn draw(&self, window: &mut RenderWindow) {
		let mut rectangle = match RectangleShape::new() {
			Some(rectangle) => rectangle,
			None() => fail!("Error, cannot create rectangle.")
		};

		rectangle.set_size(&Vector2f::new(50., 50.));
		rectangle.set_position(&self.position);
		window.draw(&rectangle);
	}
}
