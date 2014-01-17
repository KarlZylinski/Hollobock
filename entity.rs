extern mod rsfml;

use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, RectangleShape};

pub struct Entity {
	position: Vector2f
}

pub fn draw(window: &mut RenderWindow, entity: &Entity) {
	let mut rectangle = match RectangleShape::new() {
		Some(rectangle) => rectangle,
		None() => fail!("Error, cannot create rectangle.")
	};

	rectangle.set_size(&Vector2f::new(50., 50.));
	rectangle.set_position(&entity.position);
	window.draw(&rectangle);
}

pub fn update(entity: &Entity) -> Entity {
	let retval = Entity { position: entity.position };
	return retval;
}