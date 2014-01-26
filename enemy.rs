use std::f32;

use rsfml::system::Vector2f;
use rsfml::graphics::{RenderWindow, RectangleShape, FloatRect};

use entity::{Entity, UpdateResult};
use input::Input;
use world::World;
use player::Player;
use player_bullet::PlayerBullet;
use vector;
use std::iter::Iterator;

pub struct Enemy {
	position: Vector2f,
	rotation: f32
}

fn intersecting_with_bullet(enemy: &Enemy, world: &World) -> bool {
	let mut bullets = world.entities.iter().filter(|&e| (*e).is::<PlayerBullet>());

	for bullet in bullets {
		if FloatRect::intersects(
			&bullet.rect().get_global_bounds(),
			&enemy.rect().get_global_bounds(),
			&FloatRect::new(0.0,0.0,0.0,0.0)){
			
			return true;
		}
	}

	return false;
}

impl Entity for Enemy {
	fn update(&self, dt: f32, world: &World, _input: &Input) -> UpdateResult {
		let player_entity = match world.entities.iter().find(|&e| (*e).is::<Player>()) {
			Some(player) => player.as_ref::<Player>(),
			None => fail!("No player found in world.")
		};

		let player = match player_entity {
			Some(player_ref) => player_ref,
			None => fail!("Could not convert to player.")
		};

		let direction = vector::normalize(player.position - self.position);

		let new_entities = if(intersecting_with_bullet(self, world)) {
			~[]
		} else {
			~[~Enemy {
				position: self.position + direction * 100.0f32 * dt,
				rotation: f32::atan2(direction.y, direction.x)
			} as ~Entity]
		};

		return UpdateResult { new_entities: new_entities };
	}

	fn rect(&self) -> RectangleShape {
		let mut rectangle = match RectangleShape::new() {
			Some(rectangle) => rectangle,
			None() => fail!("Error, cannot create rectangle.")
		};

		let size = Vector2f::new(10., 10.);
		let origin = size * 0.5f32;

		rectangle.set_size(&size);
		rectangle.set_origin(&origin);
		rectangle.set_rotation(self.rotation.to_degrees());
		rectangle.set_position(&self.position);

		return rectangle;
	}

	fn draw(&self, window: &mut RenderWindow) {		
		window.draw(&self.rect());
	}

	fn clone(&self) -> ~Entity {
		return ~Enemy {
			position: self.position.clone(),
			rotation: self.rotation
		} as ~Entity;
	}
}
