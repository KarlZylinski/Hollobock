use std::{num, f32};

use rsfml::system::{Vector2f, Vector2i};
use rsfml::graphics::{RenderWindow, RectangleShape};
use rsfml::window::{keyboard, mouse};

use entity::{Entity, UpdateResult};
use input::{Input};

pub struct Bullet {
	position: Vector2f,
	direction: Vector2f,
	velocity: f32
}

impl Entity for Bullet {
	fn update(&self, dt: f32, _input: &Input) -> UpdateResult {
		let new_bullet = ~Bullet {
			position: self.position + self.direction * self.velocity * dt,
			direction: self.direction,
			velocity: self.velocity
		} as ~Entity;

		return UpdateResult { new_entities: ~[new_bullet] };
	}

	fn draw(&self, window: &mut RenderWindow) {
		let mut rectangle = match RectangleShape::new() {
			Some(rectangle) => rectangle,
			None() => fail!("Error, cannot create rectangle.")
		};

		let size = Vector2f::new(10., 10.);
		let origin = size * 0.5f32;

		rectangle.set_size(&size);
		rectangle.set_origin(&origin);
		rectangle.set_position(&self.position);
		window.draw(&rectangle);
	}

	fn clone(&self) -> ~Entity {
		return ~Bullet {
			position: self.position.clone(),
			direction: self.direction.clone(),
			velocity: self.velocity
		} as ~Entity;
	}
}

pub struct Player {
	position: Vector2f,
	rotation: f32,
	weapon_cooldown: f32
}

struct PlayerInput {
	direction: Vector2f,
	mouse_position: Vector2i,
	mouse_1: bool
}

fn get_input(input: &Input) -> PlayerInput {
	return PlayerInput {
		direction: Vector2f::new(
			if input.key_held(keyboard::A) {
				-1.
			} else if input.key_held(keyboard::S) {
				1.
			} else {
				0.
			},
			if input.key_held(keyboard::W) {
				-1.
			} else if input.key_held(keyboard::R) {
				1.
			} else {
				0.
			}
		),
		mouse_position: input.mouse_position,
		mouse_1: input.mouse_button_held(mouse::MouseLeft),
	};
}

fn process_weapon_input(old_cooldown: f32, dt: f32, mouse_1_down: bool) -> (f32, bool)
{
	let cooldown = num::max(0., old_cooldown - dt);

	if mouse_1_down && cooldown == 0. {
		return (0.1, true);
	}

	return (cooldown, false);
}

fn normalize(vec: Vector2f) -> Vector2f {
	let length = num::sqrt((vec.x * vec.x) + (vec.y * vec.y));
	if length != 0. {
		return Vector2f::new(vec.x / length, vec.y / length);
	} else {
		return vec;
	}
}

impl Entity for Player {
	fn update(&self, dt: f32, input: &Input) -> UpdateResult {
		let input = get_input(input);
		let new_position = self.position + input.direction * 200.0f32 * dt;
		let look_direction = Vector2f::new(input.mouse_position.x as f32 - new_position.x, input.mouse_position.y as f32 - new_position.y);
		let new_rotation = f32::atan2(look_direction.y, look_direction.x);

		let (weapon_cooldown, weapon_fired) = process_weapon_input(self.weapon_cooldown, dt, input.mouse_1);

		let new_player = ~Player {
			position: new_position,
			rotation: new_rotation,
			weapon_cooldown: weapon_cooldown,
		} as ~Entity;

		let mut new_entities = ~[new_player];

		if weapon_fired {
			new_entities.push(
				~Bullet {
					position: new_position,
					direction: normalize(look_direction),
					velocity: 400.
				} as ~Entity
			);
		}

		return UpdateResult { new_entities: new_entities };
	}

	fn draw(&self, window: &mut RenderWindow) {
		let mut rectangle = match RectangleShape::new() {
			Some(rectangle) => rectangle,
			None() => fail!("Error, cannot create rectangle.")
		};

		let size = Vector2f::new(50., 50.);
		let origin = size * 0.5f32;

		rectangle.set_size(&size);
		rectangle.set_origin(&origin);
		rectangle.set_rotation(self.rotation.to_degrees());
		rectangle.set_position(&self.position);
		window.draw(&rectangle);
	}

	fn clone(&self) -> ~Entity {
		return ~Player {
			position: self.position.clone(),
			rotation: self.rotation,
			weapon_cooldown: self.weapon_cooldown
		} as ~Entity;
	}
}
