use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, RectangleShape};
use rsfml::window::{keyboard};

use entity::{Entity};

pub struct Player {
	position: Vector2f
}

impl Entity for Player {
	fn update(&self, dt: f32) -> ~Entity {
		let mut new_player = Player { position: self.position };

		if keyboard::is_key_pressed(keyboard::Left) {
			new_player.position.x -= 200. * dt;
		} 
		if keyboard::is_key_pressed(keyboard::Right) {
			new_player.position.x += 200. * dt;
		}
		if keyboard::is_key_pressed(keyboard::Up) {
			new_player.position.y -= 200. * dt;
		}
		if keyboard::is_key_pressed(keyboard::Down) {
			new_player.position.y += 200. * dt;
		}

		return ~new_player as ~Entity;
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
